use bevy::prelude::*;

fn main() {
    App::new()
        // this crashes when closing the window with https://github.com/bevyengine/bevy/issues/11734
        // .add_systems(Startup, setup_camera)
        // this does NOT crash, instead has this: https://gist.github.com/Ablesius/f8ed4b0a7252e49d1a4380189cae19eb
        // try: replaced call to do_nothing with ()
        .add_systems(Startup, do_nothing)
        .add_plugins(DefaultPlugins)
        .run();
}

fn _setup_camera(mut commands: Commands) {
    // need to deviate from the blog entry
    // based on https://github.com/marcusbuffett/bevy_snake/commit/d7f263735dd8cc7ef5d94b961f1f7554ece780a3#r113302872
    commands.spawn(Camera2dBundle::default());
}

// TODO: drop-in
// remove as soon as _setup_camera works
fn do_nothing() {}