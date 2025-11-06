use std::path::Path;
use bevy::prelude::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_board);
    }
}

#[derive(Component)]
pub struct Board {
    pub width: i32,
    pub height: i32,
    pub cell_size: i32,
    transform: Transform,
}

impl Board {
    pub fn top_y_position(&self) -> f32 {
        self.transform.translation.y + (self.height * self.cell_size) as f32
    }

    pub fn bottom_y_position(&self) -> f32 {
        self.transform.translation.y
    }
}

fn setup_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    const BOARD_W: i32 = 9;
    const BOARD_H: i32 = 20;
    const CELL_SIZE: i32 = 32;
    let board_sprite = Sprite::from_image(asset_server.load(Path::new("sprites/board_square.png")));
    let board = Board {
        width: BOARD_W,
        height: BOARD_H,
        cell_size: CELL_SIZE,
        transform: Transform::from_xyz(
            -(BOARD_W * CELL_SIZE) as f32 / 2.,
            -(BOARD_H * CELL_SIZE) as f32 / 2.,
            0.,
        ),
    };

    for h in 0..board.height + 1 {
        for w in 0..board.width + 1 {
            let sprite = board_sprite.clone();
            let pos = board.transform.translation + vec3(
                (board.cell_size * w) as f32,
                (board.cell_size * h) as f32,
                0.,
            );
            commands.spawn((sprite, Transform::from_xyz(pos.x, pos.y, pos.z)));
        }
    }

    commands.spawn(board);
}