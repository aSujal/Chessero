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

        self.active_color = match self.active_color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        true
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
