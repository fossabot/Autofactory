use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use std::mem::size_of;
use std::mem::transmute;
use std::sync::Mutex;

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

pub trait InitializableBlockType<T>: BlockType<T> {
    fn id(&self) -> u8;
}

pub mod default;
pub mod environment;
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
    static ref BLOCK_TYPES: Mutex<
        [Option<&'static dyn InitializableBlockType<BlockData>>;
            1 << (8 * size_of::<BlockTypeId>())],
    > = Mutex::new([None; 1 << (8 * size_of::<BlockTypeId>())]);
    static ref CURRENT_ID: Mutex<BlockTypeId> = Mutex::new(0);
}

// Struct is for impls, and does not contain any data.
pub struct Blocks;

impl Blocks {
    pub fn register<F, T: InitializableBlockType<U>, U>(f: F) -> T
    where
        F: FnOnce(BlockTypeId) -> T,
    {
        let mut guard = CURRENT_ID.lock().unwrap();
        let mut btguard = BLOCK_TYPES.lock().unwrap();
        let id = *guard;
        let res = f(id);
        unsafe {
            (*btguard)[id as usize] = Some(transmute::<_, &dyn InitializableBlockType<BlockData>>(
                &res as &dyn InitializableBlockType<U>,
            ));
        }
        *guard = id + 1;
        res
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Default)]
pub struct BlockTypes;

impl Index<BlockTypeId> for BlockTypes {
    type Output = dyn InitializableBlockType<BlockData>;

    fn index(&self, i: BlockTypeId) -> &Self::Output {
        BLOCK_TYPES.lock().expect("Could not get a lock on BLOCK_TYPES.")[i as usize].expect("Invalid Block Type.")
    }
}
