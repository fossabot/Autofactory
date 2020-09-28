use super::*;
use chunkstorage::*;
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
    fn get_opt_ref<'a, T: RefType>(
        self: Ref<'a, Self, T>,
        pos: Point3D<i64>,
    ) -> Option<Ref<'a, Block, T>> {
        match self.to_wrapped() {
            NodeRef::AirLeaf(_) => None,
            NodeRef::ChunkLeaf(cl) => {
                let ChunkLeafRef { chunk, location } = cl.to_wrapped();
                chunk.get_opt_ref(pos - location.to_vector())
            }
            NodeRef::Branch(branch) => {
                let location = pos - branch.location.to_vector();
                if branch.contains(location) {
                    let trees = branch.to_wrapped().trees;
                    let x = if location.x < 0 {
                        trees.index_ref(0)
                    } else {
                        trees.index_ref(1)
                    };
                    let y = if location.y < 0 {
                        x.index_ref(0)
                    } else {
                        x.index_ref(1)
                    };
                    let z = if location.z < 0 {
                        y.index_ref(0)
                    } else {
                        y.index_ref(1)
                    };
                    z.deref_ref().get_opt_ref(location)
                } else {
                    None
                }
            }
        }
    }

    /// If this returns false, then a Branch wrapper should be created around the Octree.
    fn should_descend(&self, pos: Point3D<i64>) -> bool {
        match self {
            Node::AirLeaf(AirLeaf { size, location }) => {
                (pos - *location).to_array().iter().all(|x| x < size)
            }
            Node::ChunkLeaf(ChunkLeaf { location, .. }) => (pos - *location)
                .to_array()
                .iter()
                .all(|x| *x >= 0 && *x < CHUNK_SIZEI),
            Node::Branch(Branch { size, location, .. }) => {
                (pos - *location).abs().to_array().iter().all(|x| x < size)
            }
        }
    }
}

#[RefAccessors]
pub struct OctreeBlockStorage {
    root: Node,
}

impl BlockStorage for OctreeBlockStorage {
    fn get_opt_ref<'a, T: RefType>(
        self: Ref<'a, Self, T>,
        pos: Point3D<i64>,
    ) -> Option<Ref<'a, Block, T>> {
        self.to_wrapped().root.get_opt_ref(pos)
    }
}

impl InternalEnvironmentBlockStorage for OctreeBlockStorage {
    fn new() -> Self {
        OctreeBlockStorage {
            root: Node::AirLeaf(AirLeaf {
                location: Point3D::new(0, 0, 0),
                size: 16,
            }),
        }
    }
}

impl UnboundedBlockStorage for OctreeBlockStorage {
    fn get_mut<T>(&mut self, pos: Point3D<i64>) -> &mut Block {
        self.root.get_or_create(pos)
    }
}