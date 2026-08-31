use crate::game::{Board, GameState};

#[test]
fn fools_mate() {
    let mut board = Board::new();
    board.play_uci_sequence("f2f3 e7e5 g2g4 d8h4").unwrap();

    assert_eq!(board.game_state, GameState::Checkmate);
}
