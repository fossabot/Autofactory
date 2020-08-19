use std::rc::Rc;

use array_macro::array;

pub type BlockData = [u8; 32];

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vertex {
    x: f64,
    y: f64,
    z: f64,
}

impl Vertex {
    pub fn new(x: f64, y: f64, z: f64) -> Vertex {
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

pub trait BlockStorage {
    fn get_block(&self, coords: BlockCoords) -> &Block<BlockData>;
    fn set_block<T>(&mut self, coords: BlockCoords, block: Block<T>);
    fn iter(&self) -> Box<dyn Iterator<Item = (BlockCoords, Block<BlockData>)> + '_>;

    fn new() -> Self;
}

const CHUNK_SIZE: usize = 16;

pub struct ChunkBlockStorage {
    blocks: [[[Block<BlockData>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

use airblock::*;

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
                })
                .filter(|x| {
                    Rc::ptr_eq(&x.1.block_type, &Block::cast_type(Rc::new(AirBlockType)))
                }),
        )
    }

    fn new() -> Self {
        ChunkBlockStorage {
            blocks: array![array![array![Block::cast(Block::new(Rc::new(AirBlockType), AirBlockData)); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
        }
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
