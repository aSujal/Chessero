use super::Board;
use super::{Color, PieceType};

impl Board {
    pub fn find_king(&self, color: Color) -> Option<(usize, usize)> {
        for r in 0..8 {
            for c in 0..8 {
                if let Some(p) = self.squares[r][c] {
                    if p.piece_type == PieceType::King && p.color == color {
                        return Some((r, c));
                    }
                }
            }
        }
        None
    }

    pub fn is_in_check(&self, color: Color) -> bool {
        let (kr, kc) = self.find_king(color).unwrap();
        let enemy_color = if color == Color::White {
            Color::Black
        } else {
            Color::White
        };
        return self.is_square_attacked(kr, kc, enemy_color);
    }

    pub fn is_square_attacked(&self, row: usize, col: usize, enemy_color: Color) -> bool {
        for (dr, dc) in Self::ROOK_DIRECTIONS {
            let mut r = row as i32 + dr;
            let mut c = col as i32 + dc;
            //check if any pieces show up in the direction that would match an enemy rook or queen

            while r >= 0 && r < 8 && c >= 0 && c < 8 {
                if let Some(p) = self.squares[r as usize][c as usize] {
                    if p.color == enemy_color
                        && (p.piece_type == PieceType::Rook || p.piece_type == PieceType::Queen)
                    {
                        return true;
                    }
                    break; // if any other piece means the pieces behind are block so no need to check further
                }
                r += dr;
                c += dc;
            }
        }

        //Queen and Bishop check
        for (dr, dc) in Self::BISHOP_DIRECTIONS {
            let mut r = row as i32 + dr;
            let mut c = col as i32 + dc;

            while r >= 0 && r < 8 && c >= 0 && c < 8 {
                if let Some(p) = self.squares[r as usize][c as usize] {
                    if p.color == enemy_color
                        && (p.piece_type == PieceType::Bishop || p.piece_type == PieceType::Queen)
                    {
                        return true;
                    }
                    break;
                }
                r += dr;
                c += dc;
            }
        }

        //Knight check
        for (dr, dc) in Self::KNIGHT_OFFSETS {
            let r = row as i32 + dr;
            let c = col as i32 + dc;

            if r >= 0 && r < 8 && c >= 0 && c < 8 {
                if let Some(p) = self.squares[r as usize][c as usize] {
                    if p.color == enemy_color && (p.piece_type == PieceType::Knight) {
                        return true;
                    }
                }
            }
        }

        let pawn_directions = if enemy_color == Color::White { 1 } else { -1 };
        let rowi32 = row as i32;
        let coli32 = col as i32;
        let pawn_attacks = [
            (rowi32 + pawn_directions, coli32 + 1),
            (rowi32 + pawn_directions, coli32 - 1),
        ];
        for (r, c) in pawn_attacks {
            if r >= 0 && r < 8 && c >= 0 && c < 8 {
                if let Some(p) = self.squares[r as usize][c as usize] {
                    if p.color == enemy_color && p.piece_type == PieceType::Pawn {
                        return true;
                    }
                }
            }
        }

        for (dr, dc) in Self::KING_DIRECTIONS {
            let r = row as i32 + dr;
            let c = col as i32 + dc;

            if r >= 0 && r < 8 && c >= 0 && c < 8 {
                if let Some(p) = self.squares[r as usize][c as usize] {
                    if p.color == enemy_color && p.piece_type == PieceType::King {
                        return true;
                    }
                }
            }
        }

        false
    }
}
