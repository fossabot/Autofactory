use bevy::prelude::*;
use euclid::default::*;
use ne::entity::*;

fn main() {
    pos_test();
}
fn setup(mut commands: Commands) {
    commands.spawn((
        Position(Point3D::new(0.0, 0.0, 0.0)),
        Velocity(Vector3D::new(1.0, 0.0, 0.0)),
    ));
}
fn log_system(pos: &Position) {
    println!("{:?}", pos);
}

fn pos_test() {
    let mut app = App::build();
    app.add_plugin(bevy::type_registry::TypeRegistryPlugin::default())
        .add_plugin(bevy::core::CorePlugin::default())
        .add_plugin(bevy::transform::TransformPlugin::default())
        .add_plugin(bevy::diagnostic::DiagnosticsPlugin::default())
        .add_plugin(bevy::input::InputPlugin::default())
        .add_plugin(bevy::window::WindowPlugin::default())
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
