use std::collections::HashMap;
use std::io;

#[derive(Clone, Copy, PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq)]
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Copy)]
struct Piece {
    color: Color,
    piece_type: PieceType,
}

#[derive(Clone)]
struct Board {
    squares: [[Option<Piece>; 8]; 8],
    turn: Color,
}

impl Board {
        
}
