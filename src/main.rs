use bevy::prelude::*;

fn main() {
    App::new()
        // this crashes when closing the window
        // but behaves otherwise; see
        // https://github.com/bevyengine/bevy/issues/11734
        .add_systems(Startup, setup_camera)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
