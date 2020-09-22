use std::mem::transmute;
use std::fmt::Debug;
use std::hash::Hash;
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

impl Block {
    pub fn new(block_type: BlockTypeId, rotation: Rotation, stress: Stress) -> Block {
        Block { block_type, rotation, stress }
    }
}

pub trait BlockType<T> : Debug {
    fn new(&self, block: Block) -> T;

    /// Appends the block's mesh to the global Mesh.
    ///
    /// All values in the block should be normalized to [-0.5 to 0.5] assuming that the translation and rotation are not applied.
    /// The translation is applied first, then the rotation.
    fn append_mesh(&self, block: Block, data: &T, transform: Transform3D<f32>, mesh: &mut Mesh);
}

#[derive(Clone, Debug)]
pub struct BlockEnvironment<'a, T : Hash + Eq> {
    storage: ExternalBlockDataStorage<T>,
    block_types: &'a [&'static dyn BlockType<BlockData>; size_of::<BlockTypeId>()]
}

impl<T: Hash + Eq> BlockEnvironment<'_, T> {
    pub fn create_at(&mut self, position: T, id: BlockTypeId, rotation: Rotation, stress: Stress) -> Block {
        let block = Block { block_type: id, rotation, stress };
        let data = self.block_types[id as usize].new(block);
        self.storage.insert(position, data);
        block
    }

    pub fn add_block_type<S>(&mut self, id: BlockTypeId, block_type: &'static dyn BlockType<S>) {
        self.block_types[id as usize] = transmute(block_type);
    }

    pub fn append_mesh(&self, block: PositionedBlock<T>, transform: Transform3D<f32>, mesh: &mut Mesh) {
        let data = self.storage.get(&block.0).unwrap();
        let block = block.1;
        let block_type = self.block_types[block.block_type as usize];
        block_type.append_mesh(block, data, transform, mesh)
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
