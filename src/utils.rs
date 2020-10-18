use crate::blocks::storage::*;
use crate::blocks::*;
use chunkstorage::*;
use environment::BlockEnvironment;
use euclid::default::Point3D;
use rand::random;
use types::*;

pub fn generate_random_chunk(env: BlockEnvironment) -> ChunkBlockStorage {
    let mut chunk = ChunkBlockStorage::new(env);
    let iter = (&mut chunk).iter_mut();
    for (a, x, _) in iter {
        if random::<bool>() {
            a.create(
                x,
                example::Example.into(),
                Default::default(),
                Default::default(),
            );
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
