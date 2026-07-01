// Debug allows to print this enum to terminal
// Clone and Copy tells rust that the data is tiny so just copy the value instead of using pointers and stuff...
// PartialEq allows standard == operator so i can use piece.color == Color.White

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

// Array in rust: [type; size] so a two dimensional grid would be [[...; size]; size]
pub struct Board {
    pub squares: [[Option<Piece>; 8]; 8],
    pub active_color: Color,
}

// use impl to add functions to specific struct
impl Board {
    // -> return type in this case a Board
    pub fn new() -> Self {
        let empty_squares = [[None; 8]; 8];

        todo!("Starting position");
    }
}
