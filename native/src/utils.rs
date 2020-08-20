use crate::blocks::*;
use exampleblock::*;
use rand::prelude::*;
use std::rc::Rc;

pub fn generate_random_chunk() -> ChunkBlockStorage {
    let mut chunk = ChunkBlockStorage::new();
    for x in chunk.blocks.iter_mut() {
        for y in x.iter_mut() {
            for z in y.iter_mut() {
                if random::<f32>() > 0.5 {
                    *z = Block::cast(Block::new(Rc::new(ExampleBlockType), ExampleBlockData));
                }
            }
        }
    }
    chunk
}
