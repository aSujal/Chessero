// Debug allows to print this enum to terminal
// Clone and Copy tells rust that the data is tiny so just copy the value instead of using pointers and stuff...
// PartialEq allows standard == operator so i can use piece.color == Color.White

use serde::{Deserialize, Serialize};
//Serialize auto generates the code that turns these into JSON objects.

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

// Array in rust: [type; size] so a two dimensional grid would be [[...; size]; size]
#[derive(Serialize, Deserialize)]
pub struct Board {
    pub squares: [[Option<Piece>; 8]; 8],
    pub active_color: Color,
}

// use impl to add functions to specific struct
impl Board {
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
        };
    }

    // &self means it reads data from the current instance of the board the & means we are just borrowing it to read it and not manipulate it, i think...
    pub fn get_valid_moves(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();

        // match is like switch
        // check if there is a piece at this index
        let piece = match self.squares[row][col] {
            Some(p) => p,         // assign p to piece
            None => return moves, // if not a piece return the empty vec
        };

        if piece.color != self.active_color {
            return moves;
        }

        match piece.piece_type {
            PieceType::Pawn => {
                let direction = if piece.color == Color::White { -1 } else { 1 };
                let next_row = (row as i32 + direction) as usize;
                if next_row < 8 && self.squares[next_row][col].is_none() {
                    moves.push((next_row, col));

                    if (piece.color == Color::White && row == 6)
                        || (piece.color == Color::Black && row == 1)
                    {
                        let second_row = (row as i32 + direction + direction) as usize;
                        moves.push((second_row, col))
                    }
                }
                // do pawn captures
            }
            // do the rest
            _ => {}
        }

        moves
    }

    pub fn make_move(
        &mut self,
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
    ) -> bool {
        // double check that the desination is a valid move
        let valid_moves = self.get_valid_moves(from_row, from_col);
        if !valid_moves.contains(&(to_row, to_col)) {
            return false;
        }

        let piece = self.squares[from_row][from_col].take();

        self.squares[to_row][to_col] = piece;

        self.active_color = if self.active_color == Color::White {
            Color::Black
        } else {
            Color::White
        };

        true
    }
}
