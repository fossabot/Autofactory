use super::*;
use chunkstorage::*;
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
    /// Location of the center of the branch.
    pub location: Point3D<i64>,
    /// Each of the branches; the 0th index is the negative branch. The arrays proceed in order of `x`, `y`, and `z`.
    pub trees: [[[Box<Node>; 2]; 2]; 2],
    pub size: i64, /* Size of 1 tree branch, this is a power of two. */
}

impl Branch {
    pub fn contains(&self, location: Point3D<i64>) -> bool {
        location
            .to_vector()
            .abs()
            .to_array()
            .iter()
            .all(|x| *x < self.size)
    }
}

pub enum Node {
    ChunkLeaf(ChunkLeaf),
    AirLeaf(AirLeaf),
    Branch(Branch),
}

impl Node {
    fn get_block(&self, location: Point3D<i64>) -> Option<&Block<[u8; 32]>> {
        match self {
            Node::AirLeaf(_) => None,
            Node::ChunkLeaf(ChunkLeaf {
                chunk,
                location: chunk_location,
            }) => chunk.get_block(location - chunk_location.to_vector()),
            Node::Branch(branch) => {
                let location = location - branch.location.to_vector();
                if branch.contains(location) {
                    let trees = branch.trees;
                    let x = if location.x < 0 { trees[0] } else { trees[1] };
                    let y = if location.y < 0 { x[0] } else { x[1] };
                    let z = if location.z < 0 { y[0] } else { y[1] };
                    z.get_block(location)
                } else {
                    None
                }
            }
        }
    }
    fn add_block(&mut self, location: Point3D<i64>, block: Block<BlockData>) {

    }

    fn contains(&self, location: Point3D<i64>) -> bool {
        match self {
            Node::AirLeaf(AirLeaf { location, size }) => false,
            Node::ChunkLeaf(ChunkLeaf { location, .. }) => location.to_array().iter().all(|x| *x >= 0 && *x < CHUNK_SIZEI),
            Node::Branch(Branch { size, .. }) => location
                .to_vector()
                .abs()
                .to_array()
                .iter()
                .all(|x| x < size),
        }
    }
}

pub struct OctreeBlockStorage {
    pub root: Node,
}

impl BlockStorage for OctreeBlockStorage {
    fn get_block(&self, location: Point3D<i64>) -> Option<&Block<[u8; 32]>> {
        self.root.get_block(location)
    }
    fn set_block<T>(&mut self, location: Point3D<i64>, block: Block<T>) {
        let block = Block::cast(block);
        if !self.root.contains(location) {

        }
    }
    fn new() -> Self {
        OctreeBlockStorage {
            root: Node::ChunkLeaf(ChunkLeaf {
                location: Point3D::new(0, 0, 0),
                chunk: ChunkBlockStorage::new(),
            }),
        }
    }
}
