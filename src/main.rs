use bevy::prelude::*;

fn main() {
    App::new()
        // this crashes when closing the window
        // but behaves otherwise; see
        // https://github.com/bevyengine/bevy/issues/11734
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, spawn_snake)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct SnakeHead;

const SNAKE_HEAD_COLOUR: Color = Color::rgb(0.7, 0.7, 0.7);

fn spawn_snake(mut commands: Commands) {
    commands
        .spawn(
            SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_HEAD_COLOUR,
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(10.0, 10.0, 10.0),
                    ..default()
                },
                ..default()
            }
        )
        .insert(SnakeHead);
}