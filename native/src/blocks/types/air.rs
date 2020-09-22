use crate::blocks::*;
use default::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct AirBlockType;
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct AirBlockData;

static VERTICES: (Vec<Vertex>, Vec<u32>) = (vec![], vec![]);

impl DefaultBlockType<AirBlockData> for AirBlockType {
    fn get_vertices() -> &'static (Vec<Vertex>, Vec<u32>) {
        &VERTICES
    }
    fn new(&self, _: Block) -> AirBlockData { AirBlockData }
}
crate::assert_block_size!(AirBlockData);
