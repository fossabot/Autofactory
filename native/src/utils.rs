use crate::blocks::environment::BlockEnvironment;
use crate::blocks::storage::*;
use crate::blocks::types::example::ExampleBlock;
use chunkstorage::*;
use euclid::default::Point3D;
use neon::prelude::*;
use rand::random;

pub fn generate_random_chunk(env: BlockEnvironment) -> ChunkBlockStorage {
    let mut chunk = ChunkBlockStorage::new(env);
    let (iter, env) = (&mut chunk).iter_mut_with_env();
    for x in iter {
        if random::<bool>() {
            *x.1 = env.create_at(x.0, &*ExampleBlock, Default::default(), Default::default());
        }
    }
    chunk
}

pub fn generate_random_mesh(location: Point3D<i64>, mut mesh: &mut crate::rendering::Mesh) {
    generate_random_chunk(BlockEnvironment::new()).append_mesh(
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
