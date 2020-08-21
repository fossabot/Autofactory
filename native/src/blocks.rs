use core::iter::*;
use std::rc::Rc;

use array_macro::array;

pub type BlockData = [u8; 32];

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex { x, y, z }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct BlockCoords {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl BlockCoords {
    pub fn new(x: i64, y: i64, z: i64) -> BlockCoords {
        BlockCoords { x, y, z }
    }
}

pub trait BlockType<T>: std::fmt::Debug {
    fn get_vertices(&self, data: &T) -> Vec<Vertex>;
}

#[derive(Clone, Debug)]
pub struct Block<T> {
    pub block_type: Rc<dyn BlockType<T>>,
    pub data: BlockData,
}

impl<T> Block<T> {
    pub fn get_vertices(&self) -> Vec<Vertex> {
        unsafe {
            self.block_type
                .get_vertices(std::mem::transmute::<_, &T>(&self.data))
        }
    }

    pub fn new(block_type: Rc<dyn BlockType<T>>, data: T) -> Block<T> {
        unsafe {
            let data = *std::mem::transmute::<_, &BlockData>(&data);
            Block::<T> {
                block_type: block_type,
                data,
            }
        }
    }
    pub fn cast(block: Block<T>) -> Block<BlockData> {
        unsafe { std::mem::transmute(block) }
    }
    pub fn cast_type(t: Rc<dyn BlockType<T>>) -> Rc<dyn BlockType<BlockData>> {
        unsafe { std::mem::transmute(t) }
    }
}

pub mod airblock;
pub mod exampleblock;
// TODO: Implement Into_Iter with mut
pub trait BlockStorage {
    fn get_block(&self, coords: BlockCoords) -> &Block<BlockData>;
    fn set_block<T>(&mut self, coords: BlockCoords, block: Block<T>);
    fn iter(&self) -> Box<dyn Iterator<Item = (BlockCoords, Block<BlockData>)> + '_>;

    fn new() -> Self;
}

const CHUNK_SIZE: usize = 16;

#[derive(Clone, Debug)]
pub struct ChunkBlockStorage {
    pub blocks: [[[Block<BlockData>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

use airblock::*;

impl IntoIterator for ChunkBlockStorage {
    type Item = (BlockCoords, Block<BlockData>);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> <Self as std::iter::IntoIterator>::IntoIter {
        self.blocks
            .iter()
            .enumerate()
            .flat_map(|x| x.1.iter().enumerate().map(move |y| (x.0, y.0, y.1)))
            .flat_map(|x| x.2.iter().enumerate().map(move |y| (x.0, x.1, y.0, y.1)))
            .map(|x| {
                (
                    BlockCoords::new(x.0 as i64, x.1 as i64, x.2 as i64),
                    x.3.clone(),
                )
            })
            .collect::<Vec<Self::Item>>()
            .into_iter()
    }
}

impl BlockStorage for ChunkBlockStorage {
    fn get_block(&self, coords: BlockCoords) -> &Block<BlockData> {
        &self.blocks[coords.x as usize][coords.y as usize][coords.z as usize]
    }
    fn set_block<T>(&mut self, coords: BlockCoords, block: Block<T>) {
        self.blocks[coords.x as usize][coords.y as usize][coords.z as usize] = Block::cast(block);
    }
    fn iter(&self) -> Box<dyn Iterator<Item = (BlockCoords, Block<BlockData>)> + '_> {
        Box::new(
            self.blocks
                .iter()
                .enumerate()
                .flat_map(|x| x.1.iter().enumerate().map(move |y| (x.0, y.0, y.1)))
                .flat_map(|x| x.2.iter().enumerate().map(move |y| (x.0, x.1, y.0, y.1)))
                .map(|x| {
                    (
                        BlockCoords::new(x.0 as i64, x.1 as i64, x.2 as i64),
                        x.3.clone(),
                    )
                }),
        )
    }

    fn new() -> Self {
        ChunkBlockStorage {
            blocks: array![array![array![Block::cast(Block::new(Rc::new(AirBlockType), AirBlockData)); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }
}

impl ChunkBlockStorage {
    pub fn get_vertices(self) -> Vec<Vertex> {
        let mut vec = Vec::new();
        vec.extend(self.into_iter().flat_map(|x| {
            x.1.get_vertices().into_iter().map(move |s| {
                Vertex::new(s.x + x.0.x as f32, s.y + x.0.y as f32, s.z + x.0.z as f32)
            })
        }));
        vec
    }
}

#[macro_export]
macro_rules! assert_block_size {
    ($t:ty) => {
        static_assertions::const_assert!(
            std::mem::size_of::<$t>() <= std::mem::size_of::<BlockData>()
        );
    };
}
