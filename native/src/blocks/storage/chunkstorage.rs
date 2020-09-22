use crate::blocks::*;
use crate::rendering::Mesh;

use storage::*;

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_SIZEI: i64 = CHUNK_SIZE as i64;

#[RefAccessors]
#[derive(Clone, Debug)]
pub struct ChunkBlockStorage {
    pub blocks: Box<[[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,
}
impl BlockStorage for ChunkBlockStorage {
    fn get_opt_ref<'a, T : RefType>(this: Ref<'a, Self, T>, coords: Point3D<i64>) -> Option<Ref<'a, Block, T>> {
        if coords
            .to_array()
            .iter()
            .any(|a| *a < 0 || *a >= CHUNK_SIZEI)
        {
            None
        } else {
            Some(this.to_wrapped().blocks[coords.x as usize][coords.y as usize][coords.z as usize])
        }
    }

    fn new() -> Self {
        ChunkBlockStorage {
            blocks: Box::new(
                array![array![array![Block::new(); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            ),
        }
    }
}

impl ChunkBlockStorage {
    pub fn append_mesh(self, transform: Transform3D<f32>, mesh: &mut Mesh) {
        self.into_iter().for_each(|a| {
            a.1.append_mesh(transform.pre_translate(a.0.to_vector().to_f32()), mesh);
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
    type Item = (Point3D<i64>, Block);
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

impl IntoIterator for ChunkBlockStorage {
    type Item = (Point3D<i64>, Block);
    type IntoIter = IntoIter;
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
pub struct RefIntoIter<'a, T : RefType> {
    x: i64,
    y: i64,
    z: i64,
    chunk: Ref<'a, ChunkBlockStorage, T>,
}
// TODO: IMPLEMENT SIZE_HINT
impl<'a, T : RefType> RefIntoIter<'a, T> {
    fn step(this: &mut Ref<'a, Self, T>)  -> Option<(Point3D<i64>, Ref<'a, Block, T>)> {
        let this = this.to_wrapped();
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
        let block = this.chunk.get_opt_ref(point);
        if block.is_some() {
            Some((point, block.unwrap()))
        } else {
            None
        }
    }
}

impl<'a> Iterator for RefIntoIter<'a, Shared> {
    type Item = (Point3D<i64>, &'a Block);
    fn next(&mut self) -> Option<Self::Item> {
        self.step().map(|x| (x.0, x.1.to_ref()))
    }
}

impl<'a> Iterator for RefIntoIter<'a, Unique> {
    type Item = (Point3D<i64>, &'a mut Block);
    fn next(&mut self) -> Option<Self::Item> {
        self.step().map(|x| (x.0, x.1.to_ref()))
    }
}

impl<'a> IntoIterator for &'a ChunkBlockStorage {
    type Item = (Point3D<i64>, &'a Block);
    type IntoIter = RefIntoIter<'a, Shared>;
    fn into_iter(self) -> Self::IntoIter {
        RefIntoIter {
            x: -1,
            y: 0,
            z: 0,
            chunk: self,
        }
    }
}

impl<'a> IntoIterator for &'a mut ChunkBlockStorage {
    type Item = (Point3D<i64>, &'a mut Block);
    type IntoIter = RefIntoIter<'a, Unique>;
    fn into_iter(self) -> Self::IntoIter {
        RefIntoIter {
            x: -1,
            y: 0,
            z: 0,
            chunk: self,
        }
    }
}
