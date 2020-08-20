use crate::blocks::*;
use exampleblock::*;
use rand::prelude::*;
use std::rc::Rc;

pub fn generate_random_chunk() -> ChunkBlockStorage {
    let mut chunk = ChunkBlockStorage::new();
    for x in &mut chunk.blocks {
        for y in x {
            for z in y {
                if random::<f32>() > 0.5 {
                    *z = Block::cast(Block::new(Rc::new(ExampleBlockType), ExampleBlockData));
                }
            }
        }
    }
    chunk
}
