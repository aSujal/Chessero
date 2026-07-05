// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use crate::game::board::Board;

//
pub mod game;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_initial_board() -> Board {
    Board::new()
}

#[tauri::command]
fn get_moves(board: Board, row: usize, col: usize) -> Vec<(usize, usize)> {
    board.get_valid_moves(row, col)
}

#[tauri::command]
fn make_move(
    mut board: Board,
    from_row: usize,
    from_col: usize,
    to_row: usize,
    to_col: usize,
) -> Board {
    board.make_move(from_row, from_col, to_row, to_col);

    board
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_initial_board,
            get_moves,
            make_move
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
