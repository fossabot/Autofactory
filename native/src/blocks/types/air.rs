use crate::blocks::*;
use default::*;
use lazy_static::lazy_static;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct AirBlockType;
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct AirBlockData;

static VERTICES: (Vec<Vertex>, Vec<u32>) = (vec![], vec![]);
lazy_static! {
    pub static ref STATIC_AIR: Block<BlockData> = Block::cast(Block::new(Box::new(AirBlockType), AirBlockData));
}

impl DefaultBlockType<AirBlockData> for AirBlockType {
    fn get_vertices() -> &'static (Vec<Vertex>, Vec<u32>) {
        &VERTICES
    }
}
crate::assert_block_size!(AirBlockData);
