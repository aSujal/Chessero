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

        //this needs to be before placing back the captured piece as i could delete it if its not an en passant
        self.squares[to_r][to_c].take();

        // place the captured piece back
        if let Some((cr, cc)) = record.captured_square {
            self.squares[cr][cc] = record.captured_piece;
        }

        self.castling = record.castling_rights;
        self.en_passant_pawn = record.en_passant_pawn;

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
        }

        //Check queenside
        if to_c == 2 {
            let rook = self.squares[from_r][3].take();

            self.squares[from_r][0] = rook;
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
