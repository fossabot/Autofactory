#![allow(non_upper_case_globals)]

use crate::blocks::*;
use lazy_static::lazy_static;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Example;

lazy_static! {
    static ref VERTICES: (Vec<Vertex>, Vec<u32>) = {
        let mut vertices = vec![];
        let mut index = vec![];

        let mut compute_face = |x: Vector3D<f32>, y: Vector3D<f32>, z: Vector3D<f32>| {
            let normal = z.normalize();
            let start = vertices.len();
            vertices.extend(
                [
                    Vertex::new(Point3D::zero() - x - y + z, normal),
                    Vertex::new(Point3D::zero() + x - y + z, normal),
                    Vertex::new(Point3D::zero() - x + y + z, normal),
                    Vertex::new(Point3D::zero() + x + y + z, normal),
                ]
                .iter(),
            );
            index.extend([0, 2, 1, 2, 3, 1].iter().map(|a| a + start as u32));
        };

        let mut compute_both_faces = |x, y, z| {
            compute_face(y, x, z);
            compute_face(x, y, -z);
        };

        let x = Vector3D::new(1.0 / 2.0, 0.0, 0.0);
        let y = Vector3D::new(0.0, 1.0 / 2.0, 0.0);
        let z = Vector3D::new(0.0, 0.0, 1.0 / 2.0);

        compute_both_faces(x, y, z);
        compute_both_faces(y, z, x);
        compute_both_faces(z, x, y);

        (vertices, index)
    };
}

impl SimpleBlockType for Example {
    fn get_vertices() -> &'static (Vec<Vertex>, Vec<u32>) {
        &VERTICES
    }
}
