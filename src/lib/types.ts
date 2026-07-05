type PieceType = "pawn" | "bishop" | "knight" | "rook" | "queen" | "king";
type Color = "white" | "black";

type Piece = {
    piece_type: PieceType,
    color: Color,
}

type BoardState = {
    squares: (Piece | null)[][];
    active_color: Color
}