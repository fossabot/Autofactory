use crate::blocks::storage::*;
/*use crate::blocks::*;*/
use chunkstorage::*;
use euclid::default::Point3D;
use neon::prelude::*;
/*use rand::prelude::*;
use std::rc::Rc;
use types::example::*;*/

pub fn generate_random_chunk() -> ChunkBlockStorage {
    todo!()
    /*let mut chunk = ChunkBlockStorage::new();
    for x in &mut chunk.blocks {
        for y in x {
            for z in y {
                if random::<f32>() > 0.5 {
                    *z = Block::cast(Block::new(Rc::new(ExampleBlockType), ExampleBlockData));
                }
            }
        }
    }
    chunk*/
}

pub fn generate_random_mesh(location: Point3D<i64>, mut mesh: &mut crate::rendering::Mesh) {
    generate_random_chunk().append_mesh(
        euclid::default::Transform3D::translation(-8.0, -8.0, -8.0)
            .then_translate(location.to_f32().to_vector() * 16.0),
        &mut mesh,
    );
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

pub struct DynamicIterator<T> {
    iter: Box<dyn FnMut() -> Option<T>>
}

impl<T> Iterator for DynamicIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> { (self.iter)() }
}

impl<T> DynamicIterator<T> {
    pub fn new(a: impl FnMut() -> Option<T>) -> DynamicIterator<T> {
        DynamicIterator {
            iter: a
        }
    }
}

pub struct BorrowDynamicIterator<'a, T> {
    iter: &'a mut dyn FnMut() -> Option<T>
}

impl<T> Iterator for BorrowDynamicIterator<'_, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> { (self.iter)() }
}

impl<'a, T> BorrowDynamicIterator<'a, T> {
    pub fn new(a: &'a mut dyn FnMut() -> Option<T>) -> BorrowDynamicIterator<'a, T> {
        BorrowDynamicIterator {
            iter: a
        }
    }
}