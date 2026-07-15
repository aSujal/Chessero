use super::Board;
use super::{Color, Piece, PieceType};

impl Board {
    pub fn get_legal_moves(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut legal_moves = Vec::new();

        let pseudo_moves = self.get_pseudo_moves(row, col);

        let piece_color = match self.squares[row][col] {
            Some(p) => p.color,
            None => return legal_moves,
        };

        for (to_row, to_col) in pseudo_moves {
            let mut temp_board = self.clone();

            // Move the piece on the temp board
            // temp_board.make_move(row, col, to_row, to_col); cant use make_move because it checks for legal moves which will cause it to infinitely loop.
            temp_board.make_move_unchecked(row, col, to_row, to_col);

            // if still not in check then its a legal move
            if !temp_board.is_in_check(piece_color) {
                legal_moves.push((to_row, to_col))
            }
        }

        legal_moves
    }

    // moves that are mechanically allowed
    // &self means it reads data from the current instance of the board the & means we are just borrowing it to read it and not manipulate it, i think...
    pub fn get_pseudo_moves(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();

        // match is like switch
        // check if there is a piece at this index
        let piece = match self.squares[row][col] {
            Some(p) => p,         // assign p to piece
            None => return moves, // if not a piece return the empty vec
        };

        // not your turn
        if piece.color != self.active_color {
            return moves;
        }

        match piece.piece_type {
            PieceType::Pawn => {
                let rowi32 = row as i32; //using i32 to perform math with negetive values for directions as usize is only positive numbers.
                let coli32 = col as i32;

                let direction = if piece.color == Color::White { -1 } else { 1 };
                let next_row = (rowi32 + direction) as usize;

                if let Some((ep_row, ep_col)) = self.en_passant_pawn {
                    if ep_row == row && ep_col.abs_diff(col) == 1 {
                        moves.push(((ep_row as i32 + direction) as usize, ep_col));
                    }
                }

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
                // direction row, direction column
                for (dr, dc) in Self::KNIGHT_OFFSETS {
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
                self.handle_sliding_moves(row, col, &Self::BISHOP_DIRECTIONS, piece, &mut moves)
            }

            PieceType::Rook => {
                self.handle_sliding_moves(row, col, &Self::ROOK_DIRECTIONS, piece, &mut moves)
            }

            PieceType::Queen => {
                self.handle_sliding_moves(row, col, &Self::QUEEN_DIRECTIONS, piece, &mut moves)
            }

            PieceType::King => {
                let enemy_color = if piece.color == Color::White {
                    Color::Black
                } else {
                    Color::White
                };
                if self.can_castle_kingside(piece.color) {
                    moves.push((row, 6));
                }
                if self.can_castle_queenside(piece.color) {
                    moves.push((row, 2));
                }

                for (dr, dc) in Self::KING_DIRECTIONS {
                    let next_row = row as i32 + dr;
                    let next_col = col as i32 + dc;

                    if next_row >= 0 && next_row < 8 && next_col >= 0 && next_col < 8 {
                        let r = next_row as usize;
                        let c = next_col as usize;

                        if self.is_square_attacked(r, c, enemy_color) {
                            continue;
                        }
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

    fn can_castle_kingside(&self, color: Color) -> bool {
        let enemy_color = if color == Color::White {
            Color::Black
        } else {
            Color::White
        };

        match color {
            Color::White => {
                if self.castling.white_kingside == true {
                    if !self.is_square_attacked(7,4,enemy_color) //e1 check if the king is in check
                        && !self.is_square_attacked(7, 5, enemy_color) //f1
                        && !self.is_square_attacked(7, 6, enemy_color)
                    //g1
                    {
                        if self.squares[7][5] == None && self.squares[7][6] == None {
                            return true;
                        }
                    }
                }
            }

            Color::Black => {
                if self.castling.black_kingside == true {
                    if !self.is_square_attacked(0, 4, enemy_color)
                        && !self.is_square_attacked(0, 5, enemy_color)
                        && !self.is_square_attacked(0, 6, enemy_color)
                    {
                        if self.squares[0][5] == None && self.squares[0][6] == None {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    fn can_castle_queenside(&self, color: Color) -> bool {
        let enemy_color = if color == Color::White {
            Color::Black
        } else {
            Color::White
        };

        match color {
            Color::White => {
                if self.castling.white_queenside == true {
                    if !self.is_square_attacked(7, 4, enemy_color)
                        && !self.is_square_attacked(7, 3, enemy_color)
                        && !self.is_square_attacked(7, 2, enemy_color)
                    {
                        if self.squares[7][1] == None
                            && self.squares[7][2] == None
                            && self.squares[7][3] == None
                        {
                            return true;
                        }
                    }
                }
            }

            Color::Black => {
                if self.castling.black_queenside == true {
                    if !self.is_square_attacked(0, 4, enemy_color)
                        && !self.is_square_attacked(0, 3, enemy_color)
                        && !self.is_square_attacked(0, 2, enemy_color)
                    {
                        if self.squares[0][1] == None
                            && self.squares[0][2] == None
                            && self.squares[0][3] == None
                        {
                            return true;
                        }
                    }
                }
            }
        }

        false
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
}
