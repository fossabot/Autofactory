use crate::blocks::*;
use default::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct AirBlock;

static VERTICES: (Vec<Vertex>, Vec<u32>) = (vec![], vec![]);

impl DefaultBlock for AirBlock {
    fn get_vertices() -> &'static (Vec<Vertex>, Vec<u32>) {
        &VERTICES
    }
}
