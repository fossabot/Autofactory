use super::*;
use crate::blocks::types::air::*;
use euclid::default::Point3D;

// TODO: Implement Into_Iter with mut
pub trait BlockStorage {
    fn get_opt(&self, coords: Point3D<i64>) -> Option<&Block>;
    fn get_mut_opt(&mut self, coords: Point3D<i64>) -> Option<&mut Block>;

    fn new() -> Self;
}

pub trait UnboundedBlockStorage: BlockStorage {
    fn get_mut<T>(&mut self, coords: Point3D<i64>) -> &mut Block;
}

pub mod chunkstorage;
pub mod octreestorage;
