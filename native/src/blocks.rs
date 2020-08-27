use core::iter::*;
use std::rc::Rc;

use array_macro::array;

use crate::rendering::*;
use euclid::default::*;

pub type BlockData = [u8; 32];

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vertex {
    pub position: Point3D<f32>,
    pub normal: Vector3D<f32>,
}

impl std::ops::Add<Vector3D<f32>> for Vertex {
    type Output = Vertex;
    fn add(self, other: Vector3D<f32>) -> Self::Output {
        Vertex::new(self.position + other, self.normal)
    }
}

impl Vertex {
    pub fn new(position: Point3D<f32>, normal: Vector3D<f32>) -> Vertex {
        Vertex { position, normal }
    }
}

pub trait BlockType<T>: std::fmt::Debug {
    /// Appends the block's mesh to the global Mesh.
    ///
    /// All values in the block should be normalized to [-0.5 to 0.5] assuming that the translation and rotation are not applied.
    /// The translation is applied first, then the rotation.
    fn append_mesh(&self, data: &T, transform: Transform3D<f32>, mesh: &mut Mesh);
}

#[derive(Clone, Debug)]
pub struct Block<T> {
    pub block_type: Rc<dyn BlockType<T>>,
    data: BlockData,
}

impl<T> Block<T> {
    pub fn append_mesh(&self, transform: Transform3D<f32>, mesh: &mut Mesh) {
        unsafe {
            self.block_type
                .append_mesh(std::mem::transmute::<_, &T>(&self.data), transform, mesh)
        }
    }

    pub fn new(block_type: Rc<dyn BlockType<T>>, data: T) -> Block<T> {
        unsafe {
            let data = *std::mem::transmute::<_, &BlockData>(&data);
            Block::<T> {
                block_type: block_type,
                data,
            }
        }
    }
    pub fn cast(block: Block<T>) -> Block<BlockData> {
        unsafe { std::mem::transmute(block) }
    }
    pub fn cast_type(t: Rc<dyn BlockType<T>>) -> Rc<dyn BlockType<BlockData>> {
        unsafe { std::mem::transmute(t) }
    }
}

pub mod default;
pub mod storage;
pub mod types;

#[macro_export]
macro_rules! assert_block_size {
    ($t:ty) => {
        static_assertions::const_assert!(
            std::mem::size_of::<$t>() <= std::mem::size_of::<BlockData>()
        );
    };
}
