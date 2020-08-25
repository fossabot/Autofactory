use euclid::default::*;
use chunkstorage::*;
use super::*;
/* Every i64 has units of chunk size */
pub struct ChunkLeaf {
    pub location: Point3D<i64>,
    pub chunk: ChunkBlockStorage,
}

pub struct AirLeaf {
    pub location: Point3D<i64>,
    pub size: i64,
}

pub struct Branch {
    pub location: Point3D<i64>,
    pub trees: Box<[[[Node; 2]; 2]; 2]>,
    pub size: i64, /* Size of 1 tree branch */
}

pub enum Node {
    ChunkLeaf(ChunkLeaf),
    AirLeaf(AirLeaf),
    Branch(Branch),
}

pub struct OctreeBlockStorage {
    pub root: Node,
}

impl BlockStorage for OctreeBlockStorage {
    fn get_block(&self, _: Point3D<i64>) -> Option<&Block<[u8; 32]>> { todo!() }
    fn set_block<T>(&mut self, _: Point3D<i64>, _: Block<T>) { todo!() }
    fn new() -> Self { todo!() }
}