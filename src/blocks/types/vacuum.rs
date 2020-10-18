use crate::blocks::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Vacuum;

static VERTICES: (Vec<Vertex>, Vec<u32>) = (vec![], vec![]);

impl SimpleBlockType for Vacuum {
    fn get_vertices() -> &'static (Vec<Vertex>, Vec<u32>) {
        &VERTICES
    }
}
