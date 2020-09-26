use std::mem::transmute;
use std::cell::Cell;
use std::ops::IndexMut;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use std::mem::size_of;
use lazy_static::lazy_static;

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

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct BlockTypes([&'static dyn BlockType<BlockData>; 2 ^ (8 * size_of::<BlockTypeId>())]);

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

pub trait BlockType<T>: Debug + Sync {
    fn new(&self, block: Block) -> T;

    /// Appends the block's mesh to the global Mesh.
    ///
    /// All values in the block should be normalized to [-0.5 to 0.5] assuming that the translation and rotation are not applied.
    /// The translation is applied first, then the rotation.
    fn append_mesh(
        &self,
        block: Block,
        accessor: environment::BlockDataAccessor<T>,
        transform: Transform3D<f32>,
        mesh: &mut Mesh,
    );
}

pub mod environment;
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

lazy_static! {
    pub static ref BLOCK_TYPES: [Option<&'static dyn BlockType<BlockData>>; 2 ^ (8 * size_of::<BlockTypeId>())] = [None; 2 ^ (8 * size_of::<BlockTypeId>())];
}

static currentId: Cell<BlockTypeId> = Cell::new(0);


// Struct is for impls.
pub struct Blocks;

impl Blocks {
    pub fn register<F, T: BlockType<U>, U>(f: F) -> BlockTypeId where F: FnOnce(BlockTypeId) -> T {
        let id = currentId.get();
        BLOCK_TYPES[id as usize] = Some(transmute::<_, &dyn BlockType<BlockData>>(&f(id))); // TODO: FINISH
        currentId.replace(id + 1)
    }
}