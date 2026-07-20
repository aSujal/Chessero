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

        self.make_move_unchecked(from_row, from_col, to_row, to_col);

        true
    }

    pub fn make_move_unchecked(
        &mut self,
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
    ) {
        let moved_piece = self.squares[from_row][from_col].unwrap();
        let capture_square = self.get_capture_square(to_row, to_col, &moved_piece);
        let captured_piece = match capture_square {
            Some((row, col)) => self.squares[row][col],
            None => None,
        };
        self.save_undo_record(
            from_row,
            from_col,
            to_col,
            to_row,
            moved_piece,
            capture_square,
            captured_piece,
        );

        self.handle_capture(capture_square);

        //checks if a pawn moved 2 squares and clears previous en passant pawn
        self.update_en_passant(from_row, to_row, to_col, moved_piece);

        //remove the piece from current position
        let mut piece = self.squares[from_row][from_col].take();

        //pawn promotion
        //later let the user choose what they want instead of always a queen.
        self.promote_pawn(&mut piece, to_row, PieceType::Queen);

        //checks if it can still castle in the future by checking if the king or the rook moved
        if self.castling.white_kingside
            || self.castling.white_queenside
            || self.castling.black_kingside
            || self.castling.black_queenside
        {
            self.update_castling_rights(from_col, &moved_piece);
        }

        self.update_castling_after_capture(capture_square, captured_piece);

        self.handle_castling(from_row, from_col, to_col, &moved_piece);

        // move it to the new position (could also mean it overwrites another piece, which means it captured the piece)
        self.squares[to_row][to_col] = piece;

        //Switch turn
        self.active_color = match self.active_color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }

    fn get_capture_square(
        &self,
        to_row: usize,
        to_col: usize,
        moved_piece: &Piece,
    ) -> Option<(usize, usize)> {
        let direction = if moved_piece.color == Color::White {
            -1
        } else {
            1
        };

        //en passant
        if let Some((ep_row, ep_col)) = self.en_passant_pawn {
            let capture_row = ep_row as i32 + direction;
            if moved_piece.piece_type == PieceType::Pawn
                && to_row == capture_row as usize
                && to_col == ep_col
            {
                return Some((ep_row, ep_col));
            }
        }

        //normal capture
        if self.squares[to_row][to_col].is_some() {
            return Some((to_row, to_col));
        }

        None
    }

    fn save_undo_record(
        &mut self,
        from_row: usize,
        from_col: usize,
        to_col: usize,
        to_row: usize,
        moved_piece: Piece,
        capture_square: Option<(usize, usize)>,
        captured_piece: Option<Piece>,
    ) {
        self.move_history.push(UndoRecord {
            mv: Move {
                from: (from_row, from_col),
                to: (to_row, to_col),
            },
            moved_piece,
            captured_piece,
            captured_square: capture_square,
            castling_rights: self.castling,
            en_passant_pawn: self.en_passant_pawn,
        });
    }

    fn update_castling_after_capture(
        &mut self,
        capture_square: Option<(usize, usize)>,
        captured_piece: Option<Piece>,
    ) {
        if let Some(piece) = captured_piece {
            if piece.piece_type == PieceType::Rook {
                match capture_square {
                    Some((7, 0)) => self.castling.white_queenside = false,
                    Some((7, 7)) => self.castling.white_kingside = false,
                    Some((0, 0)) => self.castling.black_queenside = false,
                    Some((0, 7)) => self.castling.black_kingside = false,
                    _ => {}
                }
            }
        }
    }

    fn handle_capture(&mut self, capture_square: Option<(usize, usize)>) {
        if let Some((row, col)) = capture_square {
            self.squares[row][col] = None;
        }
    }

    fn update_en_passant(&mut self, from_row: usize, to_row: usize, to_col: usize, piece: Piece) {
        //clear previous en_passant_pawn
        self.en_passant_pawn = None;

        if piece.piece_type == PieceType::Pawn
            && (from_row == 1 && to_row == 3 || from_row == 6 && to_row == 4)
        {
            self.en_passant_pawn = Some((to_row, to_col));
        }
    }

    fn promote_pawn(&self, piece: &mut Option<Piece>, to_row: usize, piece_type: PieceType) {
        if let Some(p) = piece {
            if p.piece_type == PieceType::Pawn && (to_row == 0 || to_row == 7) {
                p.piece_type = piece_type;
            }
        }
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
