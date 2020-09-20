use std::mem::discriminant;
use euclid::default::*;

// u8 = u3 Axis + u2 Rot around Axis
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Rotation {
    value: u8,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Axis {
    Top = 0b000,
    Bottom = 0b001,
    North = 0b010,
    South = 0b011,
    East = 0b100,
    West = 0b101,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rot {
    Forward = 0b00000,
    Back = 0b01000,
    Left = 0b10000,
    Right = 0b11000,
}

impl Rotation {
    pub fn from(a: Axis, r: Rot) -> Rotation {
        Rotation { value: discriminant(a) & discriminant(r) }
    }

    pub fn to(self) -> (Axis, Rot) {
        unsafe {
            (self.value & 0b111 as Axis, self.value & 0b11000 as Rot)
        }
    }

    // Rotates something on the top face.
    pub fn to_transform(self) -> Transform3D<f32> { todo!() }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Vertex {
    pub position: Point3D<f32>,
    pub normal: Vector3D<f32>,
}

impl std::ops::Add<Vector3D<f32>> for Vertex {
    type Output = Vertex;
    fn add(self, other: Vector3D<f32>) -> Self::Output {
        Vertex::new(self.position + other, self.normal)
    }
}

impl Vertex {
    pub fn new(position: Point3D<f32>, normal: Vector3D<f32>) -> Vertex {
        Vertex { position, normal }
    }
}