///============================================///
///--------------------------------------------///
///           TODO: TEST THIS THING            ///
///--------------------------------------------///
///============================================///

use super::*;
use array_macro::*;
use chunkstorage::*;
use std::ops::Mul;
#[RefAccessors]
#[derive(Clone)]
pub struct ChunkLeaf {
    pub location: Point3D<i64>,
    pub chunk: ChunkBlockStorage,
}

#[RefAccessors]
#[derive(Clone)]
pub struct AirLeaf {
    pub location: Point3D<i64>,
    pub size: i64,
}

#[RefAccessors]
#[derive(Clone)]
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
        Points::all(|x| x.abs() < self.size, location)
    }
}

#[RefAccessors]
#[derive(Clone)]
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

    fn descend(&mut self, pos: Point3D<i64>) -> &mut Block {
        match self {
            Node::AirLeaf(AirLeaf { size, location }) => {
                debug_assert!(*size % CHUNK_SIZEI == 0);
                debug_assert!(((*size / CHUNK_SIZEI) as f64).log2().fract() == 0.0);
                if *size == CHUNK_SIZEI {
                    *self = Node::ChunkLeaf(ChunkLeaf {
                        location: *location,
                        chunk: ChunkBlockStorage::new(BlockEnvironment::new()),
                    });
                } else {
                    let size = *size / 2;
                    let sv = Vector3D::new(size, size, size);
                    let trees = array![|i| array![|j| array![|k| Box::new(Node::AirLeaf(AirLeaf {
                        size,
                        location: Point3D::new(i, j, k).to_i64() * Scale::new(size) - sv,
                    })); 2]; 2]; 2];
                    *self = Node::Branch(Branch {
                        location: *location + sv,
                        size,
                        trees,
                    });
                }
                self.descend(pos)
            }
            Node::ChunkLeaf(ChunkLeaf { location, chunk }) => {
                chunk.get_opt_mut(pos - location.to_vector()).unwrap()
            }
            Node::Branch(Branch {
                location, trees, ..
            }) => {
                let location = pos - location.to_vector();
                let x = if location.x < 0 {
                    &mut trees[0]
                } else {
                    &mut trees[1]
                };
                let y = if location.y < 0 { &mut x[0] } else { &mut x[1] };
                let z = if location.z < 0 { &mut y[0] } else { &mut y[1] };
                z.descend(location)
            }
        }
    }
}

#[RefAccessors]
pub struct OctreeBlockStorage {
    root: Node,
    aabb: Box3D<i64>,
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
                location: Point3D::origin(),
                size: CHUNK_SIZEI,
            }),
            aabb: Box3D::new(
                Point3D::origin(),
                Point3D::new(CHUNK_SIZEI, CHUNK_SIZEI, CHUNK_SIZEI),
            ),
        }
    }
}

impl OctreeBlockStorage {
    fn ascend(&mut self, pos: Point3D<i64>) {
        let aabb = self.aabb;
        if !aabb.contains(pos) {
            fn sign(x: i64) -> i64 {
                if x > 0 {
                    1
                } else {
                    0
                }
            }
            fn mult<T: Mul<Output = T>>(x: Point3D<T>, y: Point3D<T>) -> Point3D<T> {
                Point3D::new(x.x * y.x, x.y * y.y, x.z * y.z)
            }
            let t = pos - (aabb.min.to_vector() + aabb.max.to_vector()) / 2;
            let t = Points::map(|x| sign(x), t);
            let nt = -t + Points::repeat(1).to_vector();
            debug_assert!(aabb.width() == aabb.height() && aabb.height() == aabb.depth());
            let size = aabb.width();
            take_mut::take(&mut self.root, |root| {
                let mut root = Some(root);
                let trees = array![|i| array![|j| array![|k| Box::new({
                    let ijk = Point3D::new(i, j, k).to_i64();
                    if ijk == t {
                        root.take().unwrap()
                    } else {
                        Node::AirLeaf(AirLeaf {
                            size,
                            location: (ijk - t).to_point() * Scale::new(size) + aabb.min.to_vector(), // TODO: FIX BC WRONG
                        })
                    }
                }); 2]; 2]; 2];
                Node::Branch(Branch {
                    location: mult(aabb.min, t) + mult(aabb.max, nt).to_vector(),
                    trees,
                    size: size * 2,
                })
            });
        }
    }
}

impl UnboundedBlockStorage for OctreeBlockStorage {
    fn get_mut<T>(&mut self, pos: Point3D<i64>) -> &mut Block {
        self.ascend(pos);
        self.root.descend(pos)
    }
}

#[cfg(test)]
mod tests {
    use crate::blocks::BlockEnvironment;

    fn test_default_insertion() {
    }
}