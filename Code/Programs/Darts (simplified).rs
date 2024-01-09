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
