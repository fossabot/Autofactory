use super::*;
use euclid::default::Point3D;

// TODO: Implement Into_Iter with mut
pub trait BlockStorage {
    fn get_block(&self, coords: Point3D<i64>) -> &Block<BlockData>;
    fn set_block<T>(&mut self, coords: Point3D<i64>, block: Block<T>);
    fn iter(&self) -> Box<dyn Iterator<Item = (Point3D<i64>, Block<BlockData>)> + '_>;

    fn new() -> Self;
}

pub mod chunkstorage;
pub mod octreestorage;
