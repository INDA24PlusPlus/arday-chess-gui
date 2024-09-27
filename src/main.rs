use std::any::Any;
use std::sync::RwLockWriteGuard;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use viering_chess::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_board, draw_board, spawn_camera))
        .add_systems(
            Update,
            (spawn_pieces, handle_piece_selection, despawn_all_entities),
        )
        .run();
}
#[derive(Component)]
pub struct Board {
    pieces: Vec<Vec<char>>,
    game: Game,
    possible_moves: Vec<Position>,
}

#[derive(Component)]
pub struct ActivePiece {
    position: Vec<usize>,
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

    commands.spawn(Board {
        pieces: board,
        game,
        possible_moves: Vec::new(),
    });
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
        for rank in 0..8 {
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

                let mut possible_move = Position::new(0, 0);
                let mut found = false;

                for current_move in &board.possible_moves {
                    if (rank == (current_move.y as usize) && row == (current_move.x as usize)) {
                        possible_move = Position::new(current_move.x, current_move.y);
                        found = true;

                        break;
                    }
                }

                if piece != '-' && found == false {
                    commands.spawn((SpriteBundle {
                        transform: get_piece_position(row, rank)
                            .with_scale(Vec3::new(1.6, 1.6, 0.)),
                        texture: asset_server.load(imgPath),
                        ..default()
                    },));
                } else if (found == true) {
                    commands
                        .spawn((SpriteBundle {
                            transform: get_piece_position(row, rank),
                            texture: asset_server.load("Dot.png"),
                            ..default()
                        },))
                        .insert(Name::new("dot"));
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

pub fn get_position_from_click(position: Vec2) -> Vec<usize> {
    let mut newVec = Vec::new();

    newVec.push(((position[0] as usize) - 305) / 83);
    newVec.push(((position[1] as usize) - 25) / 83);

    newVec
}

pub fn handle_piece_selection(
    input: Res<ButtonInput<MouseButton>>,
    mut board_query: Query<&mut Board>,
    window_query: Query<&Window>,
    time: Res<Time>,
) {
    if let Ok(mut board) = board_query.get_single_mut() {
        if input.just_pressed(MouseButton::Left) {
            let window = window_query.get_single().unwrap();

            let position = get_position_from_click(window.cursor_position().unwrap());

            let moves: Vec<Position> = board
                .game
                .get_possible_moves(Position::new(position[0] as u8, position[1] as u8));

            board.possible_moves = moves;
        }
    }
}

fn despawn_all_entities(mut commands: Commands, query: Query<Entity, With<Name>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
