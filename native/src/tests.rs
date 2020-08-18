use crate::blocks::*;
use static_assertions::const_assert;
use std::mem::size_of;
use std::rc::Rc;

#[derive(Copy, Clone)]
struct ExampleBlockData {
    r: Rotation,
}
struct ExampleBlockType;
impl BlockType<ExampleBlockData> for ExampleBlockType {
    fn get_rotation(&self, data: &ExampleBlockData) -> Rotation {
        data.r
    }
}

#[test]
fn size_fits() {
    const_assert!(size_of::<ExampleBlockData>() <= size_of::<BlockData>());
}

#[test]
fn create_block_and_get_rotation() {
    let r = Rotation::Up;
    let data = ExampleBlockData { r };
    let block = Block::new(Rc::new(ExampleBlockType), data);
    assert_eq!(block.get_rotation(), ExampleBlockType.get_rotation(&data));
}
