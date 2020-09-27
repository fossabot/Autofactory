use crate::blocks::*;
use environment::BlockEnvironment;
use euclid::default::*;
use storage::chunkstorage::*;
use storage::*;
use types::*;

#[test]
fn print_vertices() {
    let mut env = BlockEnvironment::new();
    let block = env.create_at(
        Point3D::new(0, 0, 0),
        &*example::ExampleBlock,
        Default::default(),
        Default::default(),
    );
    let mut mesh = crate::rendering::Mesh::empty();
    env.append_mesh(
        (Point3D::new(0, 0, 0), block),
        Default::default(),
        &mut mesh,
    );
    println!("{:#?}", mesh);
}

#[test]
fn print_chunk() {
    let chunk = ChunkBlockStorage::new(BlockEnvironment::new());
    chunk.into_iter().for_each(|x| println!("{:?}", x));
}

#[test]
fn gen_chunk() {
    let chunk = crate::utils::generate_random_chunk(BlockEnvironment::new());
    println!("{:#?}", chunk);
}

#[test]
fn gen_vertices() {
    let chunk = crate::utils::generate_random_chunk(BlockEnvironment::new());
    let mut mesh = crate::rendering::Mesh::empty();
    chunk.append_mesh(euclid::default::Transform3D::identity(), &mut mesh);
    println!("{:?}", mesh);
}

#[test]
fn gen_random_chunk() {
    let mut mesh = crate::rendering::Mesh::empty();
    let size = 0;
    println!("{}", size);
    for x in -size..=size {
        for y in -size..=size {
            for z in -size..=size {
                crate::utils::generate_random_mesh(
                    euclid::default::Point3D::new(x, y, z),
                    &mut mesh,
                );
            }
        }
    }
    println!("{:?}", mesh);
}
