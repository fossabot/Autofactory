use crate::utils::BorrowDynamicIterator;
use crate::utils::DynamicIterator;
use crate::blocks::*;
use crate::rendering::Mesh;

use storage::*;
use types::air::*;

const CHUNK_SIZE: usize = 16;

#[derive(Clone, Debug)]
pub struct ChunkBlockStorage {
    pub blocks: Box<[[[Block<BlockData>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,
}

/*
impl IntoIterator for ChunkBlockStorage {
    type Item = (Point3D<i64>, Block<BlockData>);
    type IntoIter = DynamicIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        let mut iter = self.blocks
            .iter()
            .enumerate()
            .flat_map(|x| x.1.iter().enumerate().map(move |y| (x.0, y.0, y.1)))
            .flat_map(|x| x.2.iter().enumerate().map(move |y| (x.0, x.1, y.0, y.1)))
            .map(|x| {
                (
                    Point3D::new(x.0 as i64, x.1 as i64, x.2 as i64),
                    x.3.clone(),
                )
            });
        DynamicIterator::new(Box::new(move || iter.next()))
    }
}*/
/*
impl<'a> IntoIterator for &'a ChunkBlockStorage {
    type Item = (Point3D<i64>, &'a Block<BlockData>);
    type IntoIter = BorrowDynamicIterator<'a, Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        let iter1 = (*self.blocks)
            .iter();
        let mut iter2 =
            iter1.enumerate()
            .flat_map(|x| x.1.iter().enumerate().map(move |y| (x.0, y.0, y.1)))
            .flat_map(|x| x.2.iter().enumerate().map(move |y| (x.0, x.1, y.0, y.1)))
            .map(|x| {
                (
                    Point3D::new(x.0 as i64, x.1 as i64, x.2 as i64),
                    x.3,
                )
            });
        let mut iter = move || iter2.next();
        BorrowDynamicIterator::new(&mut iter)
    }

}
*/

impl<'a> ChunkBlockStorage {
    fn into_iter(&'a self) -> DynamicIterator<(Point3D<i64>, &'a Block<BlockData>)> {
        let iter1 = (*self.blocks)
            .iter();
        let mut iter2 =
            iter1.enumerate()
            .flat_map(|x| x.1.iter().enumerate().map(move |y| (x.0, y.0, y.1)))
            .flat_map(|x| x.2.iter().enumerate().map(move |y| (x.0, x.1, y.0, y.1)))
            .map(|x| {
                (
                    Point3D::new(x.0 as i64, x.1 as i64, x.2 as i64),
                    x.3,
                )
            });
        DynamicIterator::new(move || iter2.next())
    }

}
impl BlockStorage for ChunkBlockStorage {
    fn get_block(&self, coords: Point3D<i64>) -> Option<&Block<BlockData>> {
        if coords.to_array().iter().any(|a| *a < 0 || *a >= CHUNK_SIZE as i64) {
            None
        } else {
            Some(&self.blocks[coords.x as usize][coords.y as usize][coords.z as usize])
        }
    }
    fn set_block<T>(&mut self, coords: Point3D<i64>, block: Block<T>) {
        self.blocks[coords.x as usize][coords.y as usize][coords.z as usize] = Block::cast(block);
    }

    fn new() -> Self {
        ChunkBlockStorage {
            blocks: Box::new(array![array![array![Block::cast(Block::new(Rc::new(AirBlockType), AirBlockData)); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]),
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
