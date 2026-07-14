// Debug allows to print this enum to terminal
// Clone and Copy tells rust that the data is tiny so just copy the value instead of using pointers and stuff...
// PartialEq allows standard == operator so i can use piece.color == Color.White

use super::{CastlingRights, Color, Piece, PieceType, UndoRecord};
use serde::{Deserialize, Serialize};

//Serialize auto generates the code that turns these into JSON objects.
// Array in rust: [type; size] so a two dimensional grid would be [[...; size]; size]
#[derive(Serialize, Deserialize, Clone)]
pub struct Board {
    pub squares: [[Option<Piece>; 8]; 8],
    pub active_color: Color,
    pub move_history: Vec<UndoRecord>,
    pub castling: CastlingRights,
}

// use impl to add functions to specific struct
impl Board {
    pub const QUEEN_DIRECTIONS: [(i32, i32); 8] = [
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
    ];

    pub const ROOK_DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    pub const BISHOP_DIRECTIONS: [(i32, i32); 4] = [(-1, -1), (1, 1), (-1, 1), (1, -1)];

    pub const KING_DIRECTIONS: [(i32, i32); 8] = [
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
    ];

    pub const KNIGHT_OFFSETS: [(i32, i32); 8] = [
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
    ];

    // -> return type in this case a Board
    pub fn new() -> Self {
        let mut setup = [[None; 8]; 8];
        //place all white pawns
        for i in 0..8 {
            setup[6][i] = Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::White,
            });
        }

        setup[7][4] = Some(Piece {
            piece_type: PieceType::King,
            color: Color::White,
        });

        setup[7][3] = Some(Piece {
            piece_type: PieceType::Queen,
            color: Color::White,
        });

        setup[7][7] = Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::White,
        });

        setup[7][0] = Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::White,
        });

        setup[7][2] = Some(Piece {
            piece_type: PieceType::Bishop,
            color: Color::White,
        });

        setup[7][5] = Some(Piece {
            piece_type: PieceType::Bishop,
            color: Color::White,
        });

        setup[7][1] = Some(Piece {
            piece_type: PieceType::Knight,
            color: Color::White,
        });

        setup[7][6] = Some(Piece {
            piece_type: PieceType::Knight,
            color: Color::White,
        });

        //place all black pawns
        for i in 0..8 {
            setup[1][i] = Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::Black,
            })
        }

        setup[0][4] = Some(Piece {
            piece_type: PieceType::King,
            color: Color::Black,
        });

        setup[0][3] = Some(Piece {
            piece_type: PieceType::Queen,
            color: Color::Black,
        });

        setup[0][7] = Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::Black,
        });

        setup[0][0] = Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::Black,
        });

        setup[0][2] = Some(Piece {
            piece_type: PieceType::Bishop,
            color: Color::Black,
        });

        setup[0][5] = Some(Piece {
            piece_type: PieceType::Bishop,
            color: Color::Black,
        });

        setup[0][1] = Some(Piece {
            piece_type: PieceType::Knight,
            color: Color::Black,
        });

        setup[0][6] = Some(Piece {
            piece_type: PieceType::Knight,
            color: Color::Black,
        });

        return Board {
            squares: setup,
            active_color: Color::White,
            move_history: Vec::new(),
            castling: CastlingRights {
                white_kingside: true,
                white_queenside: true,
                black_kingside: true,
                black_queenside: true,
            },
        };
    }
}
