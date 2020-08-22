use super::default::*;
use crate::blocks::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ExampleBlockType;
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ExampleBlockData;

static mut VERTICES: Option<(Vec<Vertex>, Vec<u16>)> = None;

impl DefaultBlockType<ExampleBlockData> for ExampleBlockType {
    fn get_vertices() -> (Vec<Vertex>, Vec<u16>) {
        unsafe {
            VERTICES.clone().unwrap_or_else(|| {
                let positions = vec![
                    Point3D::new(0.5, 0.5, 0.5),
                    Point3D::new(0.5, 0.5, -0.5),
                    Point3D::new(0.5, -0.5, 0.5),
                    Point3D::new(0.5, -0.5, -0.5),
                    Point3D::new(-0.5, 0.5, -0.5),
                    Point3D::new(-0.5, 0.5, 0.5),
                    Point3D::new(-0.5, -0.5, -0.5),
                    Point3D::new(-0.5, -0.5, 0.5),
                ];

                let index = vec![
                    0, 2, 1, 2, 3, 1, 4, 6, 5, 6, 7, 5, 4, 5, 1, 5, 0, 1, 7, 6, 2, 6, 3, 2, 5, 7,
                    0, 7, 2, 0, 1, 3, 4, 3, 6, 4,
                ];

                let result = (
                    positions
                        .iter()
                        .map(|a| Vertex::new(*a, Vector3D::new(0.0, 0.0, 0.0)))
                        .collect(),
                    index,
                );
                VERTICES = Some(result.clone());
                result
            })
        }
    }
}
crate::assert_block_size!(ExampleBlockData);
