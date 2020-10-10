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

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(mut commands: Commands) {
        commands.spawn((
            Position(Point3D::new(0.0, 0.0, 0.0)),
            Velocity(Vector3D::new(1.0, 0.0, 0.0)),
        ));
    }

    fn log_system(pos: &Position) {
        println!("{:?}", pos);
    }

    #[test]
    fn pos_test() {
        let mut app = App::build();
        app.add_plugin(bevy::type_registry::TypeRegistryPlugin::default())
            .add_plugin(bevy::core::CorePlugin::default())
            .add_plugin(bevy::transform::TransformPlugin::default())
            .add_plugin(bevy::diagnostic::DiagnosticsPlugin::default())
            .add_startup_system(setup.system())
            .add_system(movement_system.system())
            .add_system(log_system.system());
        app.set_runner(|mut app| {
            app.update();
            app.update();
            app.update();
        });
        app.run();
    }
}
