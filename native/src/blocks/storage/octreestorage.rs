// TODO: MAKE THE OCTREE AUTOMATICALLY REPLACE EMPTY CHUNKS WITH AIR AND EMPTY
// BRANCHES WITH AIR TODO: FIX TAIL CALLS
use super::*;
use array_macro::*;
use chunkstorage::*;
use std::ops::Mul;
#[RefAccessors]
#[derive(Clone, Debug)]
pub struct ChunkLeaf {
    pub location: Point3D<i64>,
    pub chunk: ChunkBlockStorage,
}

#[RefAccessors]
#[derive(Clone, Debug)]
pub struct AirLeaf {
    pub location: Point3D<i64>,
    pub size: i64,
}

#[RefAccessors]
#[derive(Clone, Debug)]
pub struct Branch {
    /// Location of the center of the branch.
    pub location: Point3D<i64>,
    /// Each of the branches; the 0th index is the negative branch. The arrays
    /// proceed in order of `x`, `y`, and `z`.
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
#[derive(Clone, Debug)]
pub enum Node {
    ChunkLeaf(ChunkLeaf),
    AirLeaf(AirLeaf),
    Branch(Branch),
}

impl Node {
    fn get_opt_ref<'a, T: RefType>(
        self: Ref<'a, Self, T>,
        pos: Point3D<i64>,
    ) -> Option<(Ref<'a, Block, T>, BlockDataAccessor<'a, T>)> {
        match self.to_wrapped() {
            NodeRef::AirLeaf(_) => None,
            NodeRef::ChunkLeaf(cl) => {
                let ChunkLeafRef { chunk, location } = cl.to_wrapped();
                chunk.get_opt_env_ref(pos - location.to_vector())
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
                    z.deref_ref().get_opt_ref(pos)
                } else {
                    None
                }
            }
        }
    }

    fn descend(&mut self, pos: Point3D<i64>) -> (&mut Block, BlockDataAccessor<Unique>) {
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
                        location: Point3D::new(i, j, k).to_i64() * Scale::new(size) + location.to_vector(),
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
                chunk.get_opt_env_mut(pos - location.to_vector()).unwrap()
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
                z.descend(pos)
            }
        }
    }
}

#[RefAccessors]
#[derive(Clone, Debug)]
pub struct OctreeBlockStorage {
    root: Node,
    aabb: Box3D<i64>,
}

impl BlockStorage for OctreeBlockStorage {
    fn get_opt_env_ref<'a, T: RefType>(
        self: Ref<'a, Self, T>,
        pos: Point3D<i64>,
    ) -> Option<(Ref<'a, Block, T>, BlockDataAccessor<'a, T>)> {
        self.to_wrapped().root.get_opt_ref(pos)
    }

    type Iter<'a, T: RefType> = OctreeIter<'a, T>;

    #[allow(clippy::needless_lifetimes)]
    fn iter_ref<'a, T: RefType>(self: Ref<'a, Self, T>) -> Self::Iter<'a, T> {
        OctreeIter::new(self)
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
            let t = Points::map(sign, t);
            let nt = -t + Points::repeat(1).to_vector();
            debug_assert!(aabb.width() == aabb.height() && aabb.height() == aabb.depth());
            let size = aabb.width();
            take_mut::take(&mut self.root, |root| {
                let mut root = Some(root);
                let trees = array![|i| array![|j| array![|k| Box::new({
                    let ijk = Point3D::new(i, j, k).to_i64();
                    if ijk == nt {
                        root.take().unwrap()
                    } else {
                        Node::AirLeaf(AirLeaf {
                            size,
                            location: (ijk - nt.to_vector()) * Scale::new(size) + aabb.min.to_vector(),
                        })
                    }
                }); 2]; 2]; 2];
                Node::Branch(Branch {
                    location: mult(aabb.min, nt) + mult(aabb.max, t).to_vector(),
                    trees,
                    size: size * 2,
                })
            });
            let sv = Points::repeat(size).to_vector();
            let root = if let Node::Branch(br) = &self.root {
                br
            } else {
                panic!()
            };
            self.aabb = Box3D::new(root.location - sv, root.location + sv);
            self.ascend(pos);
        }
    }
}

impl UnboundedBlockStorage for OctreeBlockStorage {
    fn get_mut(&mut self, pos: Point3D<i64>) -> (&mut Block, BlockDataAccessor<Unique>) {
        self.ascend(pos);
        self.root.descend(pos)
    }
}

pub struct OctreeIter<'a, T: RefType> {
    tree: Ref<'a, OctreeBlockStorage, T>,
    stack: Vec<Ref<'a, Branch, T>>,
    ci: Option<ChunkIter<'a, T>>,
}

impl<'a, T: RefType> Iterator for OctreeIter<'a, T> {
    type Item = (BlockDataAccessor<'a, T>, Ref<'a, Block, T>);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a, T: RefType> OctreeIter<'a, T> {
    fn new(storage: Ref<'a, OctreeBlockStorage, T>) -> Self {
        OctreeIter {
            tree: storage,
            stack: vec![],
            ci: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::example::Example;
    #[test]
    fn test_simple_access() {
        let mut storage = OctreeBlockStorage::new();
        storage.get_mut(Point3D::new(0, 0, 0));
        assert_eq!(format!("{:?}", storage),
"OctreeBlockStorage { root: ChunkLeaf(ChunkLeaf { location: (0, 0, 0), chunk: ChunkBlockStorage(...) }), aabb: Box3D((0, 0, 0), (16, 16, 16)) }");
    }
    #[test]
    fn test_complex_access() {
        let mut storage = OctreeBlockStorage::new();
        let accessor = storage.get_mut(Point3D::new(0, 0, CHUNK_SIZEI));
        assert_eq!(format!("{:?}", accessor),
"(Block { block_type: Vacuum(Vacuum), rotation: Rotation { value: 0 }, stress: 0 }, BlockDataAccessor { location: (0, 0, 0), storage: BlockEnvironment { storage: RwLock { data: {} } }, _marker: PhantomData })");
    }
    #[test]
    fn test_block_writing() {
        let mut storage = OctreeBlockStorage::new();
        let (block, accessor) = storage.get_mut(Point3D::new(0, 0, CHUNK_SIZEI));
        accessor.rewrite(
            block,
            Example.into(),
            Default::default(),
            Default::default(),
        );
        assert_eq!(
            *block,
            Block {
                block_type: Example.into(),
                rotation: Default::default(),
                stress: Default::default()
            }
        );

        assert_eq!(format!("{:?}", storage.get_opt(Point3D::new(0, 0, CHUNK_SIZEI))),
"Some(Block { block_type: Example(Example), rotation: Rotation { value: 0 }, stress: 0 })");
        assert_eq!(None, storage.get_opt(Point3D::new(99999, 99999, 99999)));
    }
    #[test]
    fn make_big() {
        let mut storage = OctreeBlockStorage::new();
        storage.get_mut(Point3D::new(0, 0, CHUNK_SIZEI * 2));
        storage.get_opt(Point3D::new(0, 0, CHUNK_SIZEI * 2));
    }
}
