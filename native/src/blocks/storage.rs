use super::*;
use crate::blocks::types::air::*;
use euclid::default::Point3D;

// TODO: Implement Into_Iter with mut
pub trait BlockStorage {
    fn get_opt(&self, coords: Point3D<i64>) -> Option<&Block<BlockData>>;
    fn get(&self, coords: Point3D<i64>) -> &Block<BlockData> {
        self.get_opt(coords)
            .unwrap_or(&STATIC_AIR)
    }
    fn get_mut_opt(&mut self, coords: Point3D<i64>) -> Option<&mut Block<BlockData>>;

    fn new() -> Self;
}

pub trait UnboundedBlockStorage: BlockStorage {
    fn get_mut<T>(&mut self, coords: Point3D<i64>) -> &mut Block<BlockData>;
}

pub mod chunkstorage;
pub mod octreestorage;
