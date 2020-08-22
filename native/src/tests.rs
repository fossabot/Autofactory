use crate::blocks::*;
use std::rc::Rc;

#[test]
fn print_vertices() {
    let block = Block::new(
        Rc::new(example::ExampleBlockType),
        example::ExampleBlockData,
    );
    let mut mesh = crate::rendering::Mesh::empty();
    println!("{:#?}", block.append_mesh(euclid::default::Transform3D::identity(), &mut mesh));
}

#[test]
fn print_chunk() {
    let chunk = ChunkBlockStorage::new();
    chunk.into_iter().for_each(|x| println!("{:?}", x));
}

#[test]
fn gen_chunk() {
    println!("{:#?}", crate::utils::generate_random_chunk());
}

#[test]
fn gen_vertices() {
    let chunk = crate::utils::generate_random_chunk();
    let mut mesh = crate::rendering::Mesh::empty();
    chunk.append_mesh(euclid::default::Transform3D::identity(), &mut mesh);
    println!("{:?}", mesh);
}
