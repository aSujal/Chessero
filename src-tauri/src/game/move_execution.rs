use super::Board;
use super::{Color, Move, Piece, PieceType, UndoRecord};

impl Board {
    pub fn make_move(
        &mut self,
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
    ) -> bool {
        //double check that the destination is a valid move
        let valid_moves = self.get_legal_moves(from_row, from_col);
        if !valid_moves.contains(&(to_row, to_col)) {
            return false;
        }

        let moved_piece = self.squares[from_row][from_col].unwrap();
        let captured_piece = self.squares[to_row][to_col];

        let mv = Move {
            from: (from_row, from_col),
            to: (to_row, to_col),
        };

        self.move_history.push(UndoRecord {
            mv,
            moved_piece,
            captured_piece,
            castling_rights: self.castling,
        });

        //remove the piece from current position
        let mut piece = self.squares[from_row][from_col].take();

        //pawn promotion
        if let Some(ref mut p) = piece {
            if p.piece_type == PieceType::Pawn && (to_row == 0 || to_row == 7) {
                p.piece_type = PieceType::Queen; //later let the user choose what they want instead of always a queen.
            }
        }

        //checks if it can still castle in the future by checking if the king or the rook moved
        if self.castling.white_kingside
            || self.castling.white_queenside
            || self.castling.black_kingside
            || self.castling.black_queenside
        {
            self.update_castling_rights(from_col, &moved_piece);
        }

        self.handle_castling(from_row, from_col, to_col, &moved_piece);

        // move it to the new position (could also mean it overwrites another piece, which means it captured the piece)
        self.squares[to_row][to_col] = piece;

        //Switch turn
        self.active_color = match self.active_color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        true
    }

    fn update_castling_rights(&mut self, from_col: usize, moved_piece: &Piece) {
        match moved_piece.color {
            Color::White => match moved_piece.piece_type {
                PieceType::King => {
                    self.castling.white_kingside = false;
                    self.castling.white_queenside = false;
                }
                PieceType::Rook => {
                    if from_col == 7 {
                        self.castling.white_kingside = false;
                    }

                    if from_col == 0 {
                        self.castling.white_queenside = false;
                    }
                }
                _ => (),
            },
            Color::Black => match moved_piece.piece_type {
                PieceType::King => {
                    self.castling.black_kingside = false;
                    self.castling.black_queenside = false;
                }
                PieceType::Rook => {
                    if from_col == 7 {
                        self.castling.black_kingside = false;
                    }

                    if from_col == 0 {
                        self.castling.black_queenside = false;
                    }
                }
                _ => (),
            },
        }
    }

    /**
     * Moves the rook
     */
    fn handle_castling(
        &mut self,
        from_row: usize,
        from_col: usize,
        to_col: usize,
        moved_piece: &Piece,
    ) {
        if moved_piece.piece_type == PieceType::King {
            //kingside
            if from_col == 4 && to_col == 6 {
                //take the rook
                let rook = self.squares[from_row][7].take();

                self.squares[from_row][5] = rook;
            }

            //queenside
            if from_col == 4 && to_col == 2 {
                let rook = self.squares[from_row][0].take();

                self.squares[from_row][3] = rook;
            }
        }
    }
}
