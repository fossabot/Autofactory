use std::ops::IndexMut;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use std::mem::size_of;

use std::ops::Index;

use crate::rendering::*;
use euclid::default::*;

pub mod geometry;
pub use geometry::*;

pub type BlockData = [u8; 32];
pub type Stress = u16;
pub type BlockTypeId = u8;
pub type BlockLocation = Point3D<i64>;
pub type PositionedBlock = (BlockLocation, Block);
pub type ExternalBlockDataStorage = HashMap<BlockLocation, BlockData>;
pub struct BlockDataAccessor<'a, T> {
    location: BlockLocation,
    storage: &'a ExternalBlockDataStorage,
    _marker: PhantomData<T>,
}

impl<'a, T> BlockDataAccessor<'a, T> {
    pub fn access(&self) -> T {
        unsafe {
            std::mem::transmute_copy(&self.storage[&self.location])
        }
    }

    pub fn new(location: BlockLocation, storage: &'a ExternalBlockDataStorage) -> Self {
        BlockDataAccessor {
            location,
            storage,
            _marker: PhantomData,
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct BlockTypes([&'static dyn BlockType<BlockData>; size_of::<BlockTypeId>()]);

impl Index<BlockTypeId> for BlockTypes {

    type Output = &'static dyn BlockType<BlockData>;

    fn index(&self, i: BlockTypeId) -> &Self::Output {
        &self.0[i as usize]
    }
}

impl IndexMut<BlockTypeId> for BlockTypes {

    fn index_mut(&mut self, i: BlockTypeId) -> &mut Self::Output {
        &mut self.0[i as usize]
    }
}


#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Default)]
pub struct Block {
    pub block_type: BlockTypeId,
    pub rotation: Rotation,
    pub stress: Stress,
}

impl Block {
    pub fn new(block_type: BlockTypeId, rotation: Rotation, stress: Stress) -> Block {
        Block {
            block_type,
            rotation,
            stress,
        }
    }
}

pub trait BlockType<T>: Debug {
    fn new(&self, block: Block) -> T;

    /// Appends the block's mesh to the global Mesh.
    ///
    /// All values in the block should be normalized to [-0.5 to 0.5] assuming that the translation and rotation are not applied.
    /// The translation is applied first, then the rotation.
    fn append_mesh(
        &self,
        block: Block,
        accessor: BlockDataAccessor<T>,
        transform: Transform3D<f32>,
        mesh: &mut Mesh,
    );
}

#[derive(Clone, Debug)]
pub struct BlockEnvironment<'a> {
    storage: ExternalBlockDataStorage,
    block_types: &'a BlockTypes,
}

impl<'a> BlockEnvironment<'a> {
    pub fn create_at(
        &mut self,
        position: BlockLocation,
        id: BlockTypeId,
        rotation: Rotation,
        stress: Stress,
    ) -> Block {
        let block = Block {
            block_type: id,
            rotation,
            stress,
        };
        let data = self.block_types[id].new(block);
        self.storage.insert(position, data);
        block
    }

    pub fn append_mesh(
        &self,
        (position, block): PositionedBlock,
        transform: Transform3D<f32>,
        mesh: &mut Mesh,
    ) {
        let block_type = self.block_types[block.block_type];
        block_type.append_mesh(
            block,
            BlockDataAccessor::new(position, &self.storage),
            transform,
            mesh,
        )
    }

    pub fn new(block_types: &'a BlockTypes) -> Self {
        BlockEnvironment {
            storage: HashMap::new(),
            block_types,
        }
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
