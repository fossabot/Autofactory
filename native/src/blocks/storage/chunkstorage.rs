use crate::blocks::*;
use crate::rendering::Mesh;

use storage::*;
use types::air::*;

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_SIZEI: i64 = 16;

#[derive(Clone, Debug)]
pub struct ChunkBlockStorage {
    pub blocks: Box<[[[Block<BlockData>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,
}
impl BlockStorage for ChunkBlockStorage {
    fn get_opt(&self, coords: Point3D<i64>) -> Option<&Block<BlockData>> {
        if coords
            .to_array()
            .iter()
            .any(|a| *a < 0 || *a >= CHUNK_SIZEI)
        {
            None
        } else {
            Some(&self.blocks[coords.x as usize][coords.y as usize][coords.z as usize])
        }
    }
    fn get_mut_opt(&mut self, coords: Point3D<i64>) -> Option<&mut Block<BlockData>> {
        if coords
            .to_array()
            .iter()
            .any(|a| *a < 0 || *a >= CHUNK_SIZEI)
        {
            None
        } else {
            Some(&mut self.blocks[coords.x as usize][coords.y as usize][coords.z as usize])
        }
    }

    fn new() -> Self {
        ChunkBlockStorage {
            blocks: Box::new(
                array![array![array![Block::cast(Block::new(&AirBlockType, AirBlockData)); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
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
    type Item = (Point3D<i64>, Block<BlockData>);
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

pub struct BorrowIntoIter<'a> {
    x: i64,
    y: i64,
    z: i64,
    chunk: &'a ChunkBlockStorage,
}
impl<'a> Iterator for BorrowIntoIter<'a> {
    type Item = (Point3D<i64>, &'a Block<BlockData>);
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
        let block = self.chunk.get_opt(point);
        if block.is_some() {
            Some((point, block.unwrap()))
        } else {
            None
        }
    }
}
/*
struct BorrowMutIntoIter<'a> {
    x: i64,
    y: i64,
    z: i64,
    chunk: &'a mut ChunkBlockStorage,
}
impl<'a> Iterator for BorrowMutIntoIter<'a> {
    type Item = (Point3D<i64>, &'a mut Block<BlockData>);
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
        let block = self.chunk.get_mut_opt(point);
        if block.is_some() {
            Some((point, block.unwrap()))
        } else {
            None
        }
    }
}
*/
impl IntoIterator for ChunkBlockStorage {
    type Item = (Point3D<i64>, Block<BlockData>);
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

impl<'a> IntoIterator for &'a ChunkBlockStorage {
    type Item = (Point3D<i64>, &'a Block<BlockData>);
    type IntoIter = BorrowIntoIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        BorrowIntoIter {
            x: -1,
            y: 0,
            z: 0,
            chunk: self,
        }
    }
}
/*
impl<'a> IntoIterator for &'a mut ChunkBlockStorage {
    type Item = (Point3D<i64>, &'a mut Block<BlockData>);
    type IntoIter = BorrowMutIntoIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        BorrowMutIntoIter {
            x: -1,
            y: 0,
            z: 0,
            chunk: self,
        }
    }
}
*/