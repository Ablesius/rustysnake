use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(
            // f32 = u8 / U8::MAX
            Color::rgb(0.0, 0.0, 0.0),
        ))
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, spawn_snake)
        // needs to be in Update; if we leave it in
        // Startup, it'll only be executed once. Makes sense!
        .add_systems(Update, move_snake)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct SnakeHead;

// 102, 0, 204 in u8;
// it's a nice dark purple
const SNAKE_HEAD_COLOUR: Color = Color::rgb(0.4, 0.0, 0.8);

fn spawn_snake(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOUR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(SnakeHead);
}

fn move_snake(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut head_positions: Query<&mut Transform, With<SnakeHead>>,
) {
    for mut transform in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            transform.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            transform.translation.y += 2.;
        }
    }
}
