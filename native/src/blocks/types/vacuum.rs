use crate::blocks::*;
use default::*;
use lazy_static::lazy_static;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct VacuumBlock;

static VERTICES: (Vec<Vertex>, Vec<u32>) = (vec![], vec![]);

impl DefaultBlock for VacuumBlock {
    fn get_vertices() -> &'static (Vec<Vertex>, Vec<u32>) {
        &VERTICES
    }
}
