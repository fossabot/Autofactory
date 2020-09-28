use std::collections::HashMap;
use types::BlockTypes;
use std::fmt::Debug;

use std::ops::Index;

use crate::rendering::*;
use euclid::default::*;
use blocktype::*;

pub mod geometry;
pub use geometry::*;

pub type BlockData = [u8; 32];
pub type Stress = u16;
pub type BlockLocation = Point3D<i64>;
pub type PositionedBlock = (BlockLocation, Block);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct Block {
    pub block_type: BlockTypes,
    pub rotation: Rotation,
    pub stress: Stress,
}

impl Block {
    pub fn new(block_type: BlockTypes, rotation: Rotation, stress: Stress) -> Block {
        Block {
            block_type,
            rotation,
            stress,
        }
    }
}

pub mod environment;
pub mod storage;
pub mod blocktype;
pub mod types;

#[macro_export]
macro_rules! assert_block_size {
    ($t:ty) => {
        static_assertions::const_assert!(
            std::mem::size_of::<$t>() <= std::mem::size_of::<BlockData>()
        );
    };
}
