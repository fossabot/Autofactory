use std::mem::size_of;
use std::collections::HashMap;

use array_macro::array;

use crate::rendering::*;
use euclid::default::*;

pub mod geometry;
pub use geometry::*;

pub type BlockData = [u8; 32];
pub type Stress = u16;
pub type BlockTypeId = u8;
pub type PositionedBlock<T> = (T, Block);
pub type ExternalBlockDataStorage<T> = HashMap<T, BlockData>;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Default)]
pub struct Block {
    pub block_type: BlockTypeId,
    pub rotation: Rotation,
    pub stress: Stress,
}

pub trait BlockType<T> {
    fn new(&self, block: Block) -> T;

    /// Appends the block's mesh to the global Mesh.
    ///
    /// All values in the block should be normalized to [-0.5 to 0.5] assuming that the translation and rotation are not applied.
    /// The translation is applied first, then the rotation.
    fn append_mesh(&self, block: Block, data: &T, transform: Transform3D<f32>, mesh: &mut Mesh);
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct BlockEnvironment<'a, T> {
    storage: ExternalBlockDataStorage<T>,
    block_types: &'a [&'static dyn BlockType<BlockData>; size_of::<BlockTypeId>()]
}

impl<T> BlockEnvironment<'_, T> {
    pub fn create_at(&mut self, position: T, block_type: BlockTypeId, rotation: Rotation, stress: Stress) -> Block {
        let block = Block { block_type, rotation, stress };
        let data = self.block_types[block_type].create_at(block);
        self.storage.insert(position, data);
    }

    pub fn add_block_type<S>(&mut self, id: BlockTypeId, block_type: &'static dyn BlockType<S>) {
        self.block_types[id] = block_type;
    }

    pub fn append_mesh(&self, block: PositionedBlock<T>, transform: Transform3D<f32>, mesh: &mut Mesh) {
        let data = self.storage.get(block.0).unwrap();
        let block = block.1;
        let block_type = self.block_types[block.block_type];
        block_type.append_mesh(block, data, transform, mesh)
    }
}

pub mod default;
pub mod storage;
pub mod types;
