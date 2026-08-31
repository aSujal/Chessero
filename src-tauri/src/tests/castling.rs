use crate::game::{Board, Color, PieceType};
//not going to write 20 tests as this is only for learning purposes

#[test]
fn white_kingside_castle() {
    let mut board = Board::new();
    board
        .play_uci_sequence("e2e4 e7e5 g1f3 b8c6 f1c4 g8f6")
        .unwrap();

    let castled = board.make_move(7, 4, 7, 6);
    assert!(castled);

    assert!(matches!(
        board.squares[7][6],
        Some(p) if p.piece_type == PieceType::King && p.color == Color::White
    ));

    assert!(matches!(
        board.squares[7][5],
        Some(p) if p.piece_type == PieceType::Rook && p.color == Color::White
    ));

    assert!(board.squares[7][7].is_none());
}

#[test]
fn cannot_castle_through_check() {
    let mut board = Board::new();
    board
        .play_uci_sequence("e2e4 e7e5 f1c4 d7d6 g1e2 g8h6 f2f3 d8h4")
        .unwrap();

    let castled = board.make_move(7, 4, 7, 6);
    assert!(!castled);

    assert!(matches!(
        board.squares[7][4],
        Some(p) if p.piece_type == PieceType::King && p.color == Color::White
    ))
}
