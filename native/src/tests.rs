use crate::blocks::*;
use std::rc::Rc;

#[test]
fn print_vertices() {
    let block = Block::new(
        Rc::new(exampleblock::ExampleBlockType),
        exampleblock::ExampleBlockData,
    );
    println!("{:#?}", block.get_vertices());
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
    assert_ne!(chunk.get_vertices().len(), 0);
}
