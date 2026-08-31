use crate::game::{Board, Color, PieceType};

#[test]
fn en_passant() {
    let mut board = Board::new();
    board.play_uci_sequence("e2e4 h7h6 e4e5 d7d5").unwrap();

    let play_enpassant = board.make_move(3, 4, 2, 3);
    assert!(play_enpassant);

    assert!(matches!(
        board.squares[2][3],
        Some(p) if p.piece_type == PieceType::Pawn && p.color == Color::White
    ));

    assert!(board.squares[3][3].is_none()); //black pawn that was captured
    assert!(board.squares[4][4].is_none()); //white pawn
}

#[test]
fn en_passant_expired() {
    let mut board = Board::new();
    board.play_uci_sequence("e2e4 h7h6 e4e5 d7d5").unwrap();

    //ignore en passan
    board.play_uci_sequence("g1f3 a7a6").unwrap();

    let play_enpassant = board.make_move(3, 4, 2, 3);
    assert!(!play_enpassant);

    assert!(matches!(
        board.squares[3][3],
        Some(p) if p.piece_type == PieceType::Pawn && p.color == Color::Black
    ));

    assert!(matches!(
        board.squares[3][4],
        Some(p) if p.piece_type == PieceType::Pawn && p.color == Color::White
    ));
}
