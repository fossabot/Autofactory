use crate::blocks::default::*;
use crate::blocks::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct AirBlockType;
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct AirBlockData;

impl DefaultBlockType<AirBlockData> for AirBlockType {
    fn get_vertices() -> (Vec<Vertex>, Vec<u16>) {
        (vec![], vec![])
    }
}
crate::assert_block_size!(AirBlockData);
