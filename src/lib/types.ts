type PieceType = "pawn" | "bishop" | "knight" | "rook" | "queen" | "king";
type GameState = "ongoing" | "check" | "checkmate" | "statemate" | "drawfiftymove" | "drawinsufficientmaterial" | "drawrepetition";
type Color = "white" | "black";

type Piece = {
    piece_type: PieceType,
    color: Color,
}

type Move = {
    from: [number, number],
    to: [number, number],
}

type UndoRecord = {
    mv: Move,
    moved_piece: Piece,
    captured_piece: Piece | null,
    game_state: GameState
}

type BoardState = {
    squares: (Piece | null)[][];
    active_color: Color

    move_history: Array<UndoRecord>
}