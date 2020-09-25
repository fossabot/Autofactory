use crate::blocks::*;
use crate::rendering::Mesh;

use array_macro::array;
use storage::*;

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_SIZEI: i64 = CHUNK_SIZE as i64;

#[RefAccessors]
#[derive(Clone, Debug)]
pub struct ChunkBlockStorage<'a> {
    pub blocks: Box<[[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,
    pub env: BlockEnvironment<'a>,
}
impl BlockStorage for ChunkBlockStorage<'_> {
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
impl UniqueEnvironmentBlockStorage for ChunkBlockStorage<'_> {
    fn get_env(&self) -> &BlockEnvironment<'_> {
        &self.env
    }
}
impl<'a> ExternalEnvironmentBlockStorage<'a> for ChunkBlockStorage<'a> {
    fn new(mut env: BlockEnvironment<'a>) -> Self {
        let arr =                 array![|i| array![|j| array![|k| env.create_at(Point3D::new(i, j, k).to_i64(), 0, Default::default(), Default::default()); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
        ChunkBlockStorage {
            env,
            blocks: Box::new(arr),
        }
    }
}

impl ChunkBlockStorage<'_> {
    pub fn append_mesh(&self, transform: Transform3D<f32>, mesh: &mut Mesh) {
        self.into_iter().for_each(|a| {
            self.env
                .append_mesh((a.0, *a.1), transform.pre_translate(a.0.to_vector().to_f32()), mesh);
        });
    }
}

pub struct IntoIter<'a> {
    x: i64,
    y: i64,
    z: i64,
    chunk: ChunkBlockStorage<'a>,
}

impl Iterator for IntoIter<'_> {
    type Item = (BlockLocation, Block);
    fn next(&mut self) -> Option<Self::Item> {
        self.x += 1;
        if self.x >= CHUNK_SIZEI {
            self.x = 0;
            self.y += 1;
        }
        if self.y >= CHUNK_SIZEI {
            self.y = 0;
            self.z += 1;
        }
        let point = Point3D::new(self.x, self.y, self.z);
        self.chunk.get_opt(point).map(|x| (point, x.clone()))
    }
}

impl<'a> IntoIterator for ChunkBlockStorage<'a> {
    type Item = (BlockLocation, Block);
    type IntoIter = IntoIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            x: -1,
            y: 0,
            z: 0,
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
// TODO: IMPLEMENT SIZE_HINT
impl<'a, T: RefType> RefIntoIter<'a, T> {
    fn step(self: &mut Self) -> Option<(BlockLocation, Ref<'a, Block, T>)> {
        self.z += 1;
        if self.z >= CHUNK_SIZEI {
            self.z = 0;
            self.y += 1;
            self.zi = self.yi.next().unwrap().into_iter();
        }
        if self.y >= CHUNK_SIZEI {
            self.y = 0;
            self.x += 1;
            self.yi = self.xi.next().unwrap().into_iter();
        }
        if self.x >= CHUNK_SIZEI {
            None
        } else {
            Some((Point3D::new(self.x, self.y, self.z), self.zi.next().unwrap()))
        }
    }

    fn new(chunk: Ref<'a, ChunkBlockStorage<'_>, T>) -> Self {
        let chunk = chunk.to_wrapped();
        let mut xi = chunk.blocks.deref_ref().into_iter();
        let mut yi = xi.next().unwrap().into_iter();
        let zi = yi.next().unwrap().into_iter();
        RefIntoIter {
            xi,
            yi,
            zi,
            x: -1,
            y: 0,
            z: 0,
        }
    }
}

impl<'a> Iterator for RefIntoIter<'a, Shared> {
    type Item = (BlockLocation, &'a Block);
    fn next(&mut self) -> Option<Self::Item> {
        RefIntoIter::step(self).map(|x| (x.0, x.1.as_ref()))
    }
}

impl<'a> Iterator for RefIntoIter<'a, Unique> {
    type Item = (BlockLocation, &'a mut Block);
    fn next(&mut self) -> Option<Self::Item> {
        RefIntoIter::step(self).map(|mut x| (x.0, x.1.as_mut()))
    }
}

impl<'a, 'b> IntoIterator for &'a ChunkBlockStorage<'b> {
    type Item = (BlockLocation, &'a Block);
    type IntoIter = RefIntoIter<'a, Shared>;
    fn into_iter(self) -> Self::IntoIter {
        RefIntoIter::new(Ref::new(self))
    }
}

impl<'a, 'b> IntoIterator for &'a mut ChunkBlockStorage<'b> {
    type Item = (BlockLocation, &'a mut Block);
    type IntoIter = RefIntoIter<'a, Unique>;
    fn into_iter(self) -> Self::IntoIter {
        RefIntoIter::new(Ref::new(self))
    }
}
