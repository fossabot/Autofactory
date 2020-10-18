use crate::blocks::*;
use ref_clone::Unique;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Test;

impl BlockType for Test {
    fn create(self, _: Block, accessor: BlockDataAccessor<Unique>) {
        accessor.access()[0] = 0;
    }

    fn append_mesh(
        self,
        _: Block,
        _: BlockDataAccessor<Unique>,
        _: Transform3D<f32>,
        _: &mut Mesh,
    ) {
    }

    fn do_thing(self, _: Block, accessor: BlockDataAccessor<Unique>) -> String {
        let mut data = accessor.access();
        let res = if data[0] == 0 {
            "Test: Zero"
        } else {
            "Test: One"
        };
        data[0] += 1;
        res.to_string()
    }
}
