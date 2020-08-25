use crate::blocks::*;
use std::rc::Rc;
use storage::chunkstorage::*;
use storage::*;
use types::*;

#[test]
fn print_vertices() {
    let block = Block::new(
        Rc::new(example::ExampleBlockType),
        example::ExampleBlockData,
    );
    let mut mesh = crate::rendering::Mesh::empty();
    println!(
        "{:#?}",
        block.append_mesh(euclid::default::Transform3D::identity(), &mut mesh)
    );
}
/*
#[test]
fn print_chunk() {
    let chunk = ChunkBlockStorage::new();
    chunk.into_iter().for_each(|x| println!("{:?}", x));
}
*/
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

#[test]
fn gen_random_chunk() {
    let mut mesh = crate::rendering::Mesh::empty();
    let size = 3;
    println!("{}", size);
    for x in -size..=size {
        for y in -size..=size {
            for z in -size..=size {
                println!("Adding, {}, {}, {}", x, y, z);
                println!("{}", mesh.index.len());
                crate::utils::generate_random_mesh(
                    euclid::default::Point3D::new(x, y, z),
                    &mut mesh,
                );
            }
        }
    }
    println!("{:?}", mesh);
}

#[test]
fn borrow_dynamic_iterator_works() {
    let mut vec = vec![1, 2, 3];
    let mut iter = (&mut vec).into_iter();
    let iter = &mut || iter.next();
    let mut iter = crate::utils::BorrowDynamicIterator::new(iter);
    assert_eq!(*iter.next().unwrap(), 1);
}