use bevy::prelude::*;
use bevy::transform::commands;
use bevy::window::PrimaryWindow;
use viering_chess::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_board, draw_board, spawn_camera))
        .add_systems(Update, spawn_pieces)
        .run();
}

#[derive(Component)]
pub struct Board {
    pieces: Vec<Vec<char>>,
}

pub fn get_piece_position(rank: usize, file: usize) -> Transform {
    let baseX = 350.;
    let baseY = 650.;
    let horizontal_spacing = 83.;

    Transform::from_xyz(
        (baseX + ((rank as f64) * horizontal_spacing)) as f32,
        (baseY - ((file as f64) * horizontal_spacing)) as f32,
        0.,
    )
    .with_scale(Vec3::new(1.6, 1.6, 0.))
}

pub fn spawn_board(mut commands: Commands) {
    let mut game = Game::new();
    game.load_fen("rnbqkbnr/pppppp1p/8/8/6p1/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    let mut board = Vec::new();

    board.push(Vec::from(['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r']));
    board.push(Vec::from(['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p']));
    board.push(Vec::from(['-', '-', '-', '-', '-', '-', '-', '-']));
    board.push(Vec::from(['-', '-', '-', '-', '-', '-', '-', '-']));
    board.push(Vec::from(['-', '-', '-', '-', '-', '-', '-', '-']));
    board.push(Vec::from(['-', '-', '-', '-', '-', '-', '-', '-']));
    board.push(Vec::from(['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P']));
    board.push(Vec::from(['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R']));

    commands.spawn(Board { pieces: board });
}

pub fn draw_board(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0)
                .with_scale(Vec3::new(1.3, 1.3, 0.)),
            texture: asset_server.load("Board.png"),
            ..default()
        }),
    );
}

pub fn spawn_pieces(
    mut commands: Commands,
    board_query: Query<&Board>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(board) = board_query.get_single() {
        println!("has");
        for rank in 0..8 {
            println!("Run");
            for row in 0..8 {
                let piece = board.pieces[rank][row];

                let imgPath = match piece {
                    'p' => "pieces/black/pawn.png",
                    'r' => "pieces/black/rook.png",
                    'n' => "pieces/black/knight.png",
                    'b' => "pieces/black/bishop.png",
                    'q' => "pieces/black/queen.png",
                    'k' => "pieces/black/king.png",
                    'P' => "pieces/white/pawn.png",
                    'R' => "pieces/white/rook.png",
                    'N' => "pieces/white/knight.png",
                    'B' => "pieces/white/bishop.png",
                    'Q' => "pieces/white/queen.png",
                    'K' => "pieces/white/king.png",
                    _ => "",
                };

                if piece != '-' {
                    commands.spawn((SpriteBundle {
                        transform: get_piece_position(row, rank),
                        texture: asset_server.load(imgPath),
                        ..default()
                    },));
                }
            }
        }
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.),
        ..default()
    });
}
