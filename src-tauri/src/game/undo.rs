use crate::game::PieceType;
use crate::game::UndoRecord;

use super::Board;
use super::Color;

impl Board {
    pub fn undo_move(&mut self) -> bool {
        let record = match self.move_history.pop() {
            Some(r) => r,
            None => return false,
        };

        let (from_r, from_c) = record.mv.from;
        let (to_r, to_c) = record.mv.to;

        self.squares[from_r][from_c] = Some(record.moved_piece);

        // place the captured piece back
        self.squares[to_r][to_c] = record.captured_piece;

        self.castling = record.castling_rights;

        if record.moved_piece.piece_type == PieceType::King && from_c == 4 {
            self.undo_castling(&record);
        }

        self.active_color = match self.active_color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        true
    }

    fn undo_castling(&mut self, record: &UndoRecord) {
        let (from_r, _) = record.mv.from;
        let (_, to_c) = record.mv.to;

        //Check kingside
        if to_c == 6 {
            //take the rook
            let rook = self.squares[from_r][5].take();

            self.squares[from_r][7] = rook;

            // match record.moved_piece.color {
            //     Color::White => self.castling.white_kingside = true,
            //     Color::Black => self.castling.black_kingside = true,
            // }
        }

        //Check queenside
        if to_c == 2 {
            let rook = self.squares[from_r][3].take();

            self.squares[from_r][0] = rook;

            // match record.moved_piece.color {
            //     Color::White => self.castling.white_queenside = true,
            //     Color::Black => self.castling.black_queenside = true,
            // }
        }
    }

    pub fn undo_to_index(&mut self, target_index: usize) -> bool {
        if target_index >= self.move_history.len() {
            return false;
        }

        while self.move_history.len() > (target_index + 1) {
            self.undo_move();
        }

        true
    }
}
