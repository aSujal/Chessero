use super::Board;
use super::PieceType;

impl Board {
    pub fn play_uci_sequence(&mut self, moves: &str) -> Result<(), String> {
        for mv in moves.split_whitespace() {
            if !self.play_uci(mv) {
                return Err(format!("Invalid UCI move: {}", mv));
            }
        }
        Ok(())
    }

    pub fn play_uci(&mut self, uci: &str) -> bool {
        let ((from_row, from_col), (to_row, to_col)) = match self.parse_uci(uci) {
            Some(parsed) => parsed,
            None => return false,
        };

        if !self.make_move(from_row, from_col, to_row, to_col) {
            return false;
        }

        true
    }

    pub fn parse_uci(&self, uci: &str) -> Option<((usize, usize), (usize, usize))> {
        if uci.len() != 4 && uci.len() != 5 {
            return None;
        }
        let from = self.parse_square(&uci[0..2])?;
        let to = self.parse_square(&uci[2..4])?;

        // TODO: the 5 letter is promotion piece handle that later

        Some((from, to))
    }

    fn parse_square(&self, s: &str) -> Option<(usize, usize)> {
        let mut chars = s.chars();
        let file = chars.next()?;
        let rank = chars.next()?;

        if chars.next().is_some() {
            return None;
        } // too long

        //b'a': b converts it into byte (u8) which is an ASCII Value so 'a' would be 97.
        //checked_sub subtracts the files ASCII value with a. For example 'b' (98) - 'a' (97) = 1 so col index is 1
        //could also write 'a' as u8 | the difference is b'a' the compiler directly writes it as ascii, where as 'a' as u8 casts a char to u8
        let col = (file as u8).checked_sub(b'a')? as usize;
        //rank 1 is at row 7 (bottom), rank 8 is at row 0 (top)
        let row = 8 - (rank as u8).checked_sub(b'0')? as usize;

        if col > 7 || row > 7 {
            return None;
        }

        Some((row, col))
    }
}
