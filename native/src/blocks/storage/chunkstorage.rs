use crate::blocks::*;
use crate::rendering::Mesh;
use euclid::default::*;
use storage::*;
use types::air::*;

const CHUNK_SIZE: usize = 16;

#[derive(Clone, Debug)]
pub struct ChunkBlockStorage {
    pub blocks: [[[Block<BlockData>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

impl IntoIterator for ChunkBlockStorage {
    type Item = (Point3D<i64>, Block<BlockData>);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> <Self as std::iter::IntoIterator>::IntoIter {
        self.blocks
            .iter()
            .enumerate()
            .flat_map(|x| x.1.iter().enumerate().map(move |y| (x.0, y.0, y.1)))
            .flat_map(|x| x.2.iter().enumerate().map(move |y| (x.0, x.1, y.0, y.1)))
            .map(|x| {
                (
                    Point3D::new(x.0 as i64, x.1 as i64, x.2 as i64),
                    x.3.clone(),
                )
            })
            .collect::<Vec<Self::Item>>()
            .into_iter()
    }
}

impl BlockStorage for ChunkBlockStorage {
    fn get_block(&self, coords: Point3D<i64>) -> &Block<BlockData> {
        &self.blocks[coords.x as usize][coords.y as usize][coords.z as usize]
    }
    fn set_block<T>(&mut self, coords: Point3D<i64>, block: Block<T>) {
        self.blocks[coords.x as usize][coords.y as usize][coords.z as usize] = Block::cast(block);
    }
    fn iter(&self) -> Box<dyn Iterator<Item = (Point3D<i64>, Block<BlockData>)> + '_> {
        Box::new(
            self.blocks
                .iter()
                .enumerate()
                .flat_map(|x| x.1.iter().enumerate().map(move |y| (x.0, y.0, y.1)))
                .flat_map(|x| x.2.iter().enumerate().map(move |y| (x.0, x.1, y.0, y.1)))
                .map(|x| (Point3D::new(x.0, x.1, x.2).to_i64(), x.3.clone())),
        )
    }

    fn new() -> Self {
        ChunkBlockStorage {
            blocks: array![array![array![Block::cast(Block::new(Rc::new(AirBlockType), AirBlockData)); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
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
