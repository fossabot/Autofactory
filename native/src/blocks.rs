use std::rc::Rc;

use crate::make_array;

pub type BlockData = [u8; 32];
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Rotation {
    Up,
    Down,
    Forwards,
    Backwards,
    Left,
    Right,
}

pub trait BlockType<T> {
    fn get_rotation(&self, data: &T) -> Rotation;
}

#[derive(Clone)]
pub struct Block<T> {
    pub block_type: Rc<dyn BlockType<T>>,
    pub data: BlockData,
}

impl<T> Block<T> {
    pub fn get_rotation(&self) -> Rotation {
        unsafe {
            self.block_type
                .get_rotation(std::mem::transmute::<&BlockData, &T>(&self.data))
        }
    }

    pub fn new(block_type: Rc<dyn BlockType<T>>, data: T) -> Block<T> {
        unsafe {
            let data = *std::mem::transmute::<&T, &BlockData>(&data);
            Block::<T> {
                block_type: block_type,
                data,
            }
        }
    }
    pub fn cast(block: Block<T>) -> Block<BlockData> {
        unsafe { std::mem::transmute::<Block<T>, Block<BlockData>>(block) }
    }
}

pub struct AirBlockType;
pub struct AirBlockData;

impl BlockType<AirBlockData> for AirBlockType {
    fn get_rotation(&self, _: &AirBlockData) -> Rotation {
        Rotation::Up
    }
}

pub struct BlockCoords {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

pub trait BlockStorage {
    fn get_block(&self, coords: BlockCoords) -> &Block<BlockData>;
    fn set_block<T>(&mut self, coords: BlockCoords, block: Block<T>);
    fn new() -> Self;
}

const CHUNK_SIZE: usize = 16;

pub struct ChunkBlockStorage {
    blocks: [[[Block<BlockData>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

impl BlockStorage for ChunkBlockStorage {
    fn get_block(&self, coords: BlockCoords) -> &Block<BlockData> {
        &self.blocks[coords.x as usize][coords.y as usize][coords.z as usize]
    }
    fn set_block<T>(&mut self, coords: BlockCoords, block: Block<T>) {
        self.blocks[coords.x as usize][coords.y as usize][coords.z as usize] = Block::cast(block);
    }
    fn new() -> Self {
        unsafe {
            let first = |_| Block::cast(Block::new(Rc::new(AirBlockType), AirBlockData));
            let second = |_| make_array!(CHUNK_SIZE, first);
            let third = |_| make_array!(CHUNK_SIZE, second);
            ChunkBlockStorage {
                blocks: make_array!(CHUNK_SIZE, third),
            }
        }
    }
}
