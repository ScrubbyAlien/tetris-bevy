mod board;

use bevy::prelude::*;
use crate::board::Board;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(board::BoardPlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

