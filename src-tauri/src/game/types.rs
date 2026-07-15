use serde::{Deserialize, Serialize};

use super::Piece;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Move {
    pub from: (usize, usize),
    pub to: (usize, usize),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UndoRecord {
    pub mv: Move,
    pub moved_piece: Piece,
    pub captured_piece: Option<Piece>,
    pub captured_square: Option<(usize, usize)>, // Need this for en passant
    pub castling_rights: CastlingRights,
    pub en_passant_pawn: Option<(usize, usize)>,
}
