use crate::blocks::*;

#[derive(Debug)]
pub struct ExampleBlockType;
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ExampleBlockData;

impl BlockType<ExampleBlockData> for ExampleBlockType {
    fn get_vertices(&self, _: &ExampleBlockData) -> Vec<Vertex> {
        let mut vec = vec![];
        let mut split = |x: [Vertex; 4]| {
            vec.append(&mut vec![x[0], x[1], x[3], x[2], x[3], x[1]]);
        };
        let v000 = Vertex::new(0.0, 0.0, 0.0);
        let v001 = Vertex::new(0.0, 0.0, 1.0);
        let v010 = Vertex::new(0.0, 1.0, 0.0);
        let v011 = Vertex::new(0.0, 1.0, 1.0);
        let v100 = Vertex::new(1.0, 0.0, 0.0);
        let v101 = Vertex::new(1.0, 0.0, 1.0);
        let v110 = Vertex::new(1.0, 1.0, 0.0);
        let v111 = Vertex::new(1.0, 1.0, 1.0);
        split([v000, v001, v011, v010]);
        split([v100, v101, v111, v110]);

        split([v000, v100, v110, v010]);
        split([v001, v101, v111, v011]);

        split([v000, v001, v101, v100]);
        split([v010, v011, v111, v110]);

        vec
    }
}
crate::assert_block_size!(ExampleBlockData);
