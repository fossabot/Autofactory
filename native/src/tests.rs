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
    chunk.iter().for_each(|x| println!("{:?}", x));
}
