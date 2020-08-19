use crate::blocks::airblock::*;
use crate::blocks::*;
use crate::make_array;
use std::rc::Rc;

//#[test]
fn print_vertices() {
    let _block = Block::new(
        Rc::new(exampleblock::ExampleBlockType),
        exampleblock::ExampleBlockData,
    );
    //println!("{:#?}", block.get_vertices());
}

//#[test]
fn pointer_magic() {
    let x = 3;
    println!("{:#018x}", &x as *const i32 as u64);
}

#[test]
fn print_chunk() {
    //let chunk = ChunkBlockStorage::new();
    //chunk.iter().for_each(|x| println!("{:?}", x));

    unsafe {
        let items: [&Vec<u8>; 2] = std::mem::MaybeUninit::zeroed().assume_init();
        // println!("{:?}", items);
        println!("Line 29: {:#018x}", &items as *const _ as u64);
        println!("Line 30: {:#018x}", &items[0] as *const _ as u64);
        let block = Block::cast(Block::new(
            Rc::new(AirBlockType),
            AirBlockData
        ));
        println!("{:?}", block);
        make_array!(2, |_| block.clone());
        // .iter()
        // .for_each(|x| println!("{:?}", x));
    }
}
