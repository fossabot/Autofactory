use super::*;
use chunkstorage::*;
use ref_clone_derive::*;
/// Every i64 has units of chunk size
#[RefAccessors]
pub struct ChunkLeaf {
    pub location: Point3D<i64>,
    pub chunk: ChunkBlockStorage,
}

#[RefAccessors]
pub struct AirLeaf {
    pub location: Point3D<i64>,
    pub size: i64,
}

#[RefAccessors]
pub struct Branch {
    /// Location of the center of the branch.
    pub location: Point3D<i64>,
    /// Each of the branches; the 0th index is the negative branch. The arrays proceed in order of `x`, `y`, and `z`.
    pub trees: [[[Box<Node>; 2]; 2]; 2],
    /// Size of 1 tree branch. This is a power of two.
    pub size: i64,
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

#[RefAccessors]
pub enum Node {
    ChunkLeaf(ChunkLeaf),
    AirLeaf(AirLeaf),
    Branch(Branch),
}

impl Node {

    fn contains(&self, location: Point3D<i64>) -> bool {
        match self {
            Node::AirLeaf(AirLeaf { .. }) => false,
            Node::ChunkLeaf(ChunkLeaf { location: chunk_location, .. }) => (location - *chunk_location)
                .to_array()
                .iter()
                .all(|x| *x >= 0 && *x < CHUNK_SIZEI),
            Node::Branch(Branch { size, location: branch_location, .. }) => (location - *branch_location)
                .abs()
                .to_array()
                .iter()
                .all(|x| x < size),
        }
    }

    fn get_opt_ref<'a, T : RefType>(this: Ref<'a, Self, T>, coords: Point3D<i64>) -> Option<Ref<'a, Block, T>> {
        match this.to_wrapped() {
            NodeRef::AirLeaf(_) => None,
            NodeRef::ChunkLeaf(ChunkLeaf {
                chunk,
                location: chunk_location,
            }) => chunk.get_opt(location - chunk_location.to_vector()),
            Node::Branch(branch) => {
                let location = location - branch.location.to_vector();
                if branch.contains(location) {
                    let trees = &branch.trees;
                    let x = if location.x < 0 { &trees[0] } else { &trees[1] };
                    let y = if location.y < 0 { &x[0] } else { &x[1] };
                    let z = if location.z < 0 { &y[0] } else { &y[1] };
                    z.get_opt(location)
                } else {
                    None
                }
            }
        }
    }
}

pub struct OctreeBlockStorage {
    pub root: Node,
}

impl BlockStorage for OctreeBlockStorage {

    fn get_opt_ref(&mut self, location: Point3D<i64>) -> Option<&mut Block> {
        if !self.root.contains(location) {}
        self.root.get_opt_ref(location)
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
