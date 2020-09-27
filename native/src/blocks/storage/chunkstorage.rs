use crate::blocks::types::vacuum::VacuumBlock;
use crate::blocks::*;
use crate::rendering::Mesh;

use array_macro::array;
use storage::*;

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_SIZEI: i64 = CHUNK_SIZE as i64;

#[RefAccessors]
#[derive(Clone, Debug)]
pub struct ChunkBlockStorage {
    pub blocks: Box<[[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,
    pub env: BlockEnvironment,
}
impl BlockStorage for ChunkBlockStorage {
    #[allow(clippy::needless_lifetimes)]
    fn get_opt_ref<'b, T: RefType>(
        self: Ref<'b, Self, T>,
        coords: BlockLocation,
    ) -> Option<Ref<'b, Block, T>> {
        if coords
            .to_array()
            .iter()
            .any(|a| *a < 0 || *a >= CHUNK_SIZEI)
        {
            None
        } else {
            Some(
                self.to_wrapped()
                    .blocks
                    .deref_ref()
                    .index_ref(coords.x as usize)
                    .index_ref(coords.y as usize)
                    .index_ref(coords.z as usize),
            )
        }
    }
}
impl UniqueEnvironmentBlockStorage for ChunkBlockStorage {
    #[allow(clippy::needless_lifetimes)]
    fn get_env_ref<'a, T: RefType>(self: Ref<'a, Self, T>) -> Ref<'a, BlockEnvironment, T> {
        self.to_wrapped().env
    }
}
impl ExternalEnvironmentBlockStorage for ChunkBlockStorage {
    fn new(mut env: BlockEnvironment) -> Self {
        let arr = array![|i| array![|j| array![|k| env.create_at(Point3D::new(i, j, k).to_i64(), &*VacuumBlock, Default::default(), Default::default()); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
        ChunkBlockStorage {
            env,
            blocks: Box::new(arr),
        }
    }
}

impl ChunkBlockStorage {
    pub fn append_mesh(&self, transform: Transform3D<f32>, mesh: &mut Mesh) {
        self.into_iter().for_each(|a| {
            self.env.append_mesh(
                (a.0, *a.1),
                transform.pre_translate(a.0.to_vector().to_f32()),
                mesh,
            );
        });
    }
}

pub struct IntoIter {
    x: i64,
    y: i64,
    z: i64,
    chunk: ChunkBlockStorage,
}

impl Iterator for IntoIter {
    type Item = (BlockLocation, Block);
    fn next(&mut self) -> Option<Self::Item> {
        self.z += 1;
        if self.z >= CHUNK_SIZEI {
            self.z = 0;
            self.y += 1;
        }
        if self.y >= CHUNK_SIZEI {
            self.y = 0;
            self.x += 1;
        }
        let point = Point3D::new(self.x, self.y, self.z);
        self.chunk.get_opt(point).map(|a| (point, *a))
    }
}

impl IntoIterator for ChunkBlockStorage {
    type Item = (BlockLocation, Block);
    type IntoIter = IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            x: 0,
            y: 0,
            z: -1,
            chunk: self,
        }
    }
}

#[RefAccessors]
pub struct RefIntoIter<'a, T: RefType> {
    xi: RefIter<'a, [[Block; CHUNK_SIZE]; CHUNK_SIZE], T>,
    yi: RefIter<'a, [Block; CHUNK_SIZE], T>,
    zi: RefIter<'a, Block, T>,
    x: i64,
    y: i64,
    z: i64,
}
impl<'a, T: RefType> RefIntoIter<'a, T> {
    fn step(&mut self) -> Option<(BlockLocation, Ref<'a, Block, T>)> {
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
            Point3D::new(self.x, self.y, self.z),
            self.zi.next().unwrap(),
        ))
    }

    fn new(blocks: Ref<'a, Box<[[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>, T>) -> Self {
        let mut xi = blocks.deref_ref().into_iter();
        let mut yi = xi.next().unwrap().into_iter();
        let zi = yi.next().unwrap().into_iter();
        RefIntoIter {
            xi,
            yi,
            zi,
            x: 0,
            y: 0,
            z: -1,
        }
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

impl<'a> Iterator for RefIntoIter<'a, Shared> {
    type Item = (BlockLocation, &'a Block);
    fn next(&mut self) -> Option<Self::Item> {
        RefIntoIter::step(self).map(|x| (x.0, x.1.as_ref()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        RefIntoIter::size_hint(self)
    }
}
impl<'a> ExactSizeIterator for RefIntoIter<'a, Shared> {}

impl<'a> Iterator for RefIntoIter<'a, Unique> {
    type Item = (BlockLocation, &'a mut Block);
    fn next(&mut self) -> Option<Self::Item> {
        RefIntoIter::step(self).map(|mut x| (x.0, x.1.as_mut()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        RefIntoIter::size_hint(self)
    }
}
impl<'a> ExactSizeIterator for RefIntoIter<'a, Unique> {}

impl<'a> IntoIterator for &'a ChunkBlockStorage {
    type Item = (BlockLocation, &'a Block);
    type IntoIter = RefIntoIter<'a, Shared>;
    fn into_iter(self) -> Self::IntoIter {
        RefIntoIter::new(Ref::new(&self.blocks))
    }
}

impl<'a> IntoIterator for &'a mut ChunkBlockStorage {
    type Item = (BlockLocation, &'a mut Block);
    type IntoIter = RefIntoIter<'a, Unique>;
    fn into_iter(self) -> Self::IntoIter {
        RefIntoIter::new(Ref::new(&mut self.blocks))
    }
}

impl ChunkBlockStorage {
    pub fn iter_mut_with_env<'a>(&'a mut self) -> (RefIntoIter<'a, Unique>, &mut BlockEnvironment) {
        (RefIntoIter::new(Ref::new(&mut self.blocks)), &mut self.env)
    }
}
