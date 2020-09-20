use std::mem::size_of;
use std::collections::HashMap;
use std::marker::PhantomData;

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
    fn append_mesh(&self, block: Block, data: &T, transform: Transform3D<f32>, mesh: &mut Mesh);
}

// u8 = u3 Axis + u2 Rot around Axis
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Rotation {
    value: u8,
}
pub type Stress = u16;
pub type BlockTypeId = u8;
pub type PositionedBlock = (Point3D<i64>, Block);

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Block {
    pub block_type: BlockTypeId,
    pub rotation: Rotation,
    pub stress: Stress,
    _private_marker: PhantomData<u8>
}

pub type ExternalBlockDataStorage = HashMap<Point3D<i64>, BlockData>;

impl Block {
    pub fn new(block_type: BlockTypeId, rotation: Rotation, stress: Stress) -> Block {
        Block {
            block_type,
            rotation,
            stress,
            _private_marker: PhantomData,
        }
    }
}

pub struct BlockEnvironment {
    storage: ExternalBlockDataStorage,
    block_types: [&'static dyn BlockType<BlockData>; size_of::<BlockTypeId>()]
}

impl BlockEnvironment {
    pub fn append_mesh(block: PositionedBlock, transform: Transform3D<f32>, mesh: &mut Mesh) {

    }
}