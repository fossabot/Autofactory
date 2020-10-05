use bevy::prelude::*;
use euclid::default::*;
use floating_duration::TimeAsFloat;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Position(pub Point3D<f32>);
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Velocity(pub Vector3D<f32>);

pub fn movement_system(dt: Res<Time>, mut pos: Mut<Position>, vel: &Velocity) {
    pos.0 += vel.0 * (dt.delta.as_fractional_secs() as f32);
}
