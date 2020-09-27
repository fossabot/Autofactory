#![allow(non_upper_case_globals)]

use crate::blocks::*;
use default::*;
use lazy_static::lazy_static;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Vacuum(u8);
lazy_static! {
    pub static ref VacuumBlock: Vacuum = Blocks::register(|x| Vacuum(x));
}

impl InitializableBlockType<DefaultBlockData> for Vacuum {
    fn id(&self) -> u8 {
        self.0
    }
}

static VERTICES: (Vec<Vertex>, Vec<u32>) = (vec![], vec![]);

impl DefaultBlock for Vacuum {
    fn get_vertices() -> &'static (Vec<Vertex>, Vec<u32>) {
        &VERTICES
    }
}
