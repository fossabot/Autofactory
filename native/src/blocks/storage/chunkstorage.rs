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
    fn get_env(&self) -> BlockEnvironment<'_> {
        self.env
    }
}
impl<'a> ExternalEnvironmentBlockStorage<'a> for ChunkBlockStorage<'a> {
    fn new(env: BlockEnvironment<'a>) -> Self {
        ChunkBlockStorage {
            env,
            blocks: Box::new(
                array![|i| array![|j| array![|k| env.create_at(Point3D::new(i, j, k).to_i64(), 0, Default::default(), Default::default()); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            ),
        }
    }
}

impl ChunkBlockStorage<'_> {
    pub fn append_mesh(self, transform: Transform3D<f32>, mesh: &mut Mesh) {
        self.into_iter().for_each(|a| {
            self.env
                .append_mesh(a, transform.pre_translate(a.0.to_vector().to_f32()), mesh);
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
pub struct RefIntoIter<'a, 'b, T: RefType> {
    x: i64,
    y: i64,
    z: i64,
    chunk: Ref<'a, ChunkBlockStorage<'b>, T>,
}
// TODO: IMPLEMENT SIZE_HINT
impl<'a, 'b, T: RefType> RefIntoIter<'a, 'b, T> {
    fn step(this: &mut Self) -> Option<(BlockLocation, Ref<'a, Block, T>)> {
        this.x += 1;
        if this.x >= CHUNK_SIZEI {
            this.x = 0;
            this.y += 1;
        }
        if this.y >= CHUNK_SIZEI {
            this.y = 0;
            this.z += 1;
        }
        let point = Point3D::new(this.x, this.y, this.z);
        let block = BlockStorage::get_opt_ref(this.chunk, point);
        if block.is_some() {
            Some((point, block.unwrap()))
        } else {
            None
        }
    }
}

impl<'a, 'b> Iterator for RefIntoIter<'a, 'b, Shared> {
    type Item = (BlockLocation, &'a Block);
    fn next(&mut self) -> Option<Self::Item> {
        RefIntoIter::step(self).map(|x| (x.0, x.1.as_ref()))
    }
}

impl<'a, 'b> Iterator for RefIntoIter<'a, 'b, Unique> {
    type Item = (BlockLocation, &'a mut Block);
    fn next(&mut self) -> Option<Self::Item> {
        RefIntoIter::step(self).map(|x| (x.0, x.1.as_mut()))
    }
}

impl<'a, 'b> IntoIterator for &'a ChunkBlockStorage<'b> {
    type Item = (BlockLocation, &'a Block);
    type IntoIter = RefIntoIter<'a, 'b, Shared>;
    fn into_iter(self) -> Self::IntoIter {
        RefIntoIter {
            x: -1,
            y: 0,
            z: 0,
            chunk: Ref::new(self),
        }
    }
}

impl<'a, 'b> IntoIterator for &'a mut ChunkBlockStorage<'b> {
    type Item = (BlockLocation, &'a mut Block);
    type IntoIter = RefIntoIter<'a, 'b, Unique>;
    fn into_iter(self) -> Self::IntoIter {
        RefIntoIter {
            x: -1,
            y: 0,
            z: 0,
            chunk: Ref::new(self),
        }
    }
}
