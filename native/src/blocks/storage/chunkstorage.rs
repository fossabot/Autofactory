use crate::blocks::*;
use crate::rendering::Mesh;
use std::fmt::Debug;

use array_macro::array;
use storage::*;

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_SIZEI: i64 = CHUNK_SIZE as i64;

#[RefAccessors]
#[derive(Clone)]
pub struct ChunkBlockStorage {
    pub blocks: Box<[[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,
    pub env: BlockEnvironment,
}

impl Debug for ChunkBlockStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("ChunkBlockStorage(...)")
    }
}

impl ChunkBlockStorage {
    fn get_wrapped<'a, T: RefType>(
        blocks: Ref<'a, Box<[[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>, T>,
        coords: BlockLocation,
    ) -> Option<Ref<'a, Block, T>> {
        if Points::any(|a| a < 0 || a >= CHUNK_SIZEI, coords) {
            None
        } else {
            Some(
                blocks
                    .deref_ref()
                    .index_ref(coords.x as usize)
                    .index_ref(coords.y as usize)
                    .index_ref(coords.z as usize),
            )
        }
    }
}

impl BlockStorage for ChunkBlockStorage {
    fn get_opt_ref<'a, T: RefType>(
        self: Ref<'a, Self, T>,
        coords: Point3D<i64>,
    ) -> Option<Ref<'a, Block, T>> {
        Self::get_wrapped(self.to_wrapped().blocks, coords)
    }

    fn get_opt_env_ref<'a, T: RefType>(
        self: Ref<'a, Self, T>,
        coords: Point3D<i64>,
    ) -> Option<(Ref<'a, Block, T>, BlockDataAccessor<'a, T>)> {
        let ChunkBlockStorageRef { blocks, env } = self.to_wrapped();
        match Self::get_wrapped(blocks, coords) {
            Some(a) => Some((a, BlockDataAccessor::new(coords, env.as_ref()))),
            None => None,
        }
    }
    type Iter<'a, T: RefType> = ChunkIter<'a, T>;
    fn iter_ref<'a, T: RefType>(self: Ref<'a, Self, T>) -> Self::Iter<'a, T> {
        ChunkIter::new(self)
    }
}
impl UniqueEnvironmentBlockStorage for ChunkBlockStorage {
    #[allow(clippy::needless_lifetimes)]
    fn get_env_ref<'a, T: RefType>(self: Ref<'a, Self, T>) -> Ref<'a, BlockEnvironment, T> {
        self.to_wrapped().env
    }
}
impl ExternalEnvironmentBlockStorage for ChunkBlockStorage {
    fn new(env: BlockEnvironment) -> Self {
        let arr = array![|i|
                array![|j|
                    array![|k|
                        env.create_at(
                            Point3D::new(i, j, k).to_i64(),
                            Default::default(),
                            Default::default(),
                            Default::default());
                    CHUNK_SIZE];
                CHUNK_SIZE];
            CHUNK_SIZE];
        ChunkBlockStorage {
            env,
            blocks: Box::new(arr),
        }
    }
}

impl ChunkBlockStorage {
    pub fn append_mesh(&mut self, transform: Transform3D<f32>, mesh: &mut Mesh) {
        let iter = self.iter_mut();
        iter.for_each(|(accessor, a)| {
            let loc = accessor.location().to_vector().to_f32();
            a.block_type.append_mesh(
                *a,
                accessor,
                transform.pre_translate(loc),
                mesh,
            );
        });
    }
}

#[RefAccessors]
pub struct ChunkIter<'a, T: RefType> {
    env: &'a BlockEnvironment, // TODO: FINISH
    xi: RefIter<'a, [[Block; CHUNK_SIZE]; CHUNK_SIZE], T>,
    yi: RefIter<'a, [Block; CHUNK_SIZE], T>,
    zi: RefIter<'a, Block, T>,
    x: i64,
    y: i64,
    z: i64,
}
impl<'a, T: RefType> Iterator for ChunkIter<'a, T> {
    type Item = (BlockDataAccessor<'a, T>, Ref<'a, Block, T>);
    fn next(&mut self) -> Option<Self::Item> {
        self.z += 1;
        if self.z >= CHUNK_SIZEI {
            self.z = 0;
            self.y += 1;
            if self.y >= CHUNK_SIZEI {
                self.y = 0;
                self.x += 1;
                if self.x >= CHUNK_SIZEI {
                    return None;
                }
                self.yi = self.xi.next().unwrap().into_iter();
            }
            self.zi = self.yi.next().unwrap().into_iter();
        }
        Some((
            BlockDataAccessor::new(Point3D::new(self.x, self.y, self.z), self.env),
            self.zi.next().unwrap(),
        ))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = CHUNK_SIZEI * CHUNK_SIZEI * CHUNK_SIZEI
            - self.z
            - self.y * CHUNK_SIZEI
            - self.x * CHUNK_SIZEI * CHUNK_SIZEI
            - 1;
        (len as usize, Some(len as usize))
    }
}

impl<'a, T: RefType> ChunkIter<'a, T> {
    fn new(
        storage: Ref<'a, ChunkBlockStorage, T>
    ) -> Self {
        let ChunkBlockStorageRef { blocks, env } = storage.to_wrapped();
        let env = env.as_ref();
        let mut xi = blocks.deref_ref().into_iter();
        let mut yi = xi.next().unwrap().into_iter();
        let zi = yi.next().unwrap().into_iter();
        ChunkIter {
            env,
            xi,
            yi,
            zi,
            x: 0,
            y: 0,
            z: -1,
        }
    }
}
