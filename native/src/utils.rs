use crate::blocks::*;
use example::*;
use neon::prelude::*;
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

pub fn to_buffer<'a, T>(cx: &mut CallContext<'a, JsObject>, vec: Vec<T>) -> JsResult<'a, JsBuffer> {
    unsafe {
        let slice = vec.as_slice();
        let slice = std::slice::from_raw_parts(
            slice.as_ptr() as *const u8,
            slice.len() * std::mem::size_of::<T>(),
        );
        let mut buf = cx.buffer(slice.len() as u32).unwrap();
        cx.borrow_mut(&mut buf, |data| {
            data.as_mut_slice::<u8>().copy_from_slice(slice)
        });
        Ok(buf)
    }
}
