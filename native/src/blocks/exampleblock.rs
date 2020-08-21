use crate::blocks::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ExampleBlockType;
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ExampleBlockData;

impl BlockType<ExampleBlockData> for ExampleBlockType {
    fn get_vertices(&self, _: &ExampleBlockData) -> Vec<Vertex> {
        let mut vec = vec![];
        let vertices = [
            Vertex::new(0.5, 0.5, 0.5),
            Vertex::new(0.5, 0.5, -0.5),
            Vertex::new(0.5, -0.5, 0.5),
            Vertex::new(0.5, -0.5, -0.5),
            Vertex::new(-0.5, 0.5, -0.5),
            Vertex::new(-0.5, 0.5, 0.5),
            Vertex::new(-0.5, -0.5, -0.5),
            Vertex::new(-0.5, -0.5, 0.5),
        ];
        let mut at = |x: usize| {
            vec.push(vertices[x]);
        };

        let index = [
            0, 2, 1, 2, 3, 1, 4, 6, 5, 6, 7, 5, 4, 5, 1, 5, 0, 1, 7, 6, 2, 6, 3, 2, 5, 7, 0, 7, 2,
            0, 1, 3, 4, 3, 6, 4,
        ];
        for x in index.iter() {
            at(*x);
        }

        vec
    }
}
crate::assert_block_size!(ExampleBlockData);
