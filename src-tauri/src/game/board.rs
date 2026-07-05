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
    const QUEEN_DIRECTIONS: [(i32, i32); 8] = [
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
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
                let rowi32 = row as i32; //using i32 to perform math with negetive values for directions as usize is only positive numbers.
                let coli32 = col as i32;

                let direction = if piece.color == Color::White { -1 } else { 1 };
                let next_row = (rowi32 + direction) as usize;
                if next_row < 8 && self.squares[next_row][col].is_none() {
                    moves.push((next_row, col));

                    if (piece.color == Color::White && row == 6)
                        || (piece.color == Color::Black && row == 1)
                    {
                        let second_row = (rowi32 + direction + direction) as usize;
                        if self.squares[second_row][col].is_none() {
                            moves.push((second_row, col))
                        }
                    }
                }
                // do pawn captures

                let capture_cols = [coli32 - 1, coli32 + 1];
                if next_row < 8 {
                    for target_col in capture_cols {
                        if target_col >= 0 && target_col < 8 {
                            let c = target_col as usize; // convert back to usize so i can use it as an index to find it in array

                            if let Some(target_piece) = self.squares[next_row][c] {
                                if target_piece.color != piece.color {
                                    moves.push((next_row, c));
                                }
                            }
                        }
                    }
                }
            }

            PieceType::Knight => {
                let offsets = [
                    (2, 1),
                    (2, -1),
                    (-2, 1),
                    (-2, -1),
                    (1, 2),
                    (1, -2),
                    (-1, 2),
                    (-1, -2),
                ];
                // direction row, direction column
                for (dr, dc) in offsets {
                    let new_row = row as i32 + dr;
                    let new_col = col as i32 + dc;

                    if new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8 {
                        let r = new_row as usize;
                        let c = new_col as usize;

                        match self.squares[r][c] {
                            None => moves.push((r, c)),
                            Some(target_piece) => {
                                if target_piece.color != piece.color {
                                    moves.push((r, c));
                                }
                            }
                        }
                    }
                }
            }

            PieceType::Bishop => {
                let directions = [(-1, -1), (1, 1), (-1, 1), (1, -1)];
                self.handle_sliding_moves(row, col, &directions, piece, &mut moves)
            }

            PieceType::Rook => {
                let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                self.handle_sliding_moves(row, col, &directions, piece, &mut moves)
            }

            PieceType::Queen => {
                self.handle_sliding_moves(row, col, &Self::QUEEN_DIRECTIONS, piece, &mut moves)
            }

            PieceType::King => {
                let directions = [
                    (-1, -1),
                    (1, 1),
                    (-1, 1),
                    (1, -1),
                    (-1, 0),
                    (1, 0),
                    (0, -1),
                    (0, 1),
                ];
                for (dr, dc) in directions {
                    let next_row = row as i32 + dr;
                    let next_col = col as i32 + dc;

                    if next_row >= 0 && next_row < 8 && next_col >= 0 && next_col < 8 {
                        let r = next_row as usize;
                        let c = next_col as usize;
                        //check if that square is within the range of an enemy piece and if so skip
                        match self.squares[r][c] {
                            None => moves.push((r, c)),
                            Some(target_piece) => {
                                if target_piece.color != piece.color {
                                    moves.push((r, c));
                                }
                            }
                        }
                    }
                }
            }
        }

        moves
    }

    fn handle_sliding_moves(
        &self,
        row: usize,
        col: usize,
        directions: &[(i32, i32)],
        piece: Piece,
        moves: &mut Vec<(usize, usize)>,
    ) {
        for (dr, dc) in directions {
            let mut current_row = row as i32 + dr;
            let mut current_col = col as i32 + dc;

            while current_row >= 0 && current_row < 8 && current_col >= 0 && current_col < 8 {
                let r = current_row as usize;
                let c = current_col as usize;

                match self.squares[r][c] {
                    None => {
                        moves.push((r, c));
                        current_row += dr;
                        current_col += dc;
                    }
                    Some(target_piece) => {
                        if target_piece.color != piece.color {
                            moves.push((r, c));
                        }
                        break;
                    }
                }
            }
        }
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
