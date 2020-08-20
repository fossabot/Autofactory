use crate::blocks::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct AirBlockType;
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct AirBlockData;

impl BlockType<AirBlockData> for AirBlockType {
    fn get_vertices(&self, _: &AirBlockData) -> Vec<Vertex> {
        vec![]
    }
}
crate::assert_block_size!(AirBlockData);
