use euclid::default::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Mesh {
    pub index: Vec<u16>,
    pub positions: Vec<Point3D<f32>>,
    pub normals: Vec<Vector3D<f32>>,
}

impl Mesh {
    pub fn new(index: Vec<u16>, positions: Vec<Point3D<f32>>, normals: Vec<Vector3D<f32>>) -> Mesh {
        Mesh {
            index,
            positions,
            normals,
        }
    }

    pub fn empty() -> Mesh {
        Self::new(vec![], vec![], vec![])
    }
}