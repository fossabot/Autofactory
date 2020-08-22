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
    pub data: BlockData,
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

pub mod air;
pub mod default;
pub mod example;

// TODO: Implement Into_Iter with mut
pub trait BlockStorage {
    fn get_block(&self, coords: Point3D<i64>) -> &Block<BlockData>;
    fn set_block<T>(&mut self, coords: Point3D<i64>, block: Block<T>);
    fn iter(&self) -> Box<dyn Iterator<Item = (Point3D<i64>, Block<BlockData>)> + '_>;

    fn new() -> Self;
}

const CHUNK_SIZE: usize = 16;

#[derive(Clone, Debug)]
pub struct ChunkBlockStorage {
    pub blocks: [[[Block<BlockData>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

use air::*;

impl IntoIterator for ChunkBlockStorage {
    type Item = (Point3D<i64>, Block<BlockData>);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> <Self as std::iter::IntoIterator>::IntoIter {
        self.blocks
            .iter()
            .enumerate()
            .flat_map(|x| x.1.iter().enumerate().map(move |y| (x.0, y.0, y.1)))
            .flat_map(|x| x.2.iter().enumerate().map(move |y| (x.0, x.1, y.0, y.1)))
            .map(|x| {
                (
                    Point3D::new(x.0 as i64, x.1 as i64, x.2 as i64),
                    x.3.clone(),
                )
            })
            .collect::<Vec<Self::Item>>()
            .into_iter()
    }
}

impl BlockStorage for ChunkBlockStorage {
    fn get_block(&self, coords: Point3D<i64>) -> &Block<BlockData> {
        &self.blocks[coords.x as usize][coords.y as usize][coords.z as usize]
    }
    fn set_block<T>(&mut self, coords: Point3D<i64>, block: Block<T>) {
        self.blocks[coords.x as usize][coords.y as usize][coords.z as usize] = Block::cast(block);
    }
    fn iter(&self) -> Box<dyn Iterator<Item = (Point3D<i64>, Block<BlockData>)> + '_> {
        Box::new(
            self.blocks
                .iter()
                .enumerate()
                .flat_map(|x| x.1.iter().enumerate().map(move |y| (x.0, y.0, y.1)))
                .flat_map(|x| x.2.iter().enumerate().map(move |y| (x.0, x.1, y.0, y.1)))
                .map(|x| (Point3D::new(x.0, x.1, x.2).to_i64(), x.3.clone())),
        )
    }

    fn new() -> Self {
        ChunkBlockStorage {
            blocks: array![array![array![Block::cast(Block::new(Rc::new(AirBlockType), AirBlockData)); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }
}

impl ChunkBlockStorage {
    pub fn append_mesh(self, transform: Transform3D<f32>, mesh: &mut Mesh) {
        self.into_iter().for_each(|a| {
            a.1.append_mesh(transform.pre_translate(a.0.to_vector().to_f32()), mesh);
        });
    }
}

#[macro_export]
macro_rules! assert_block_size {
    ($t:ty) => {
        static_assertions::const_assert!(
            std::mem::size_of::<$t>() <= std::mem::size_of::<BlockData>()
        );
    };
}
