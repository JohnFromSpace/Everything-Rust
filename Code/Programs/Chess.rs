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
    fn new() -> Board {
        Board {
            squares: [[None; 8]; 8],
            turn: Color::White,
        }
    }

    fn init(&mut self) {
        // Initialize the board with pieces in their starting positions
        self.add_piece(0, 0, PieceType::Rook, Color::White);
        self.add_piece(0, 1, PieceType::Knight, Color::White);
        self.add_piece(0, 2, PieceType::Bishop, Color::White);
        self.add_piece(0, 3, PieceType::Queen, Color::White);
        self.add_piece(0, 4, PieceType::King, Color::White);
        self.add_piece(0, 5, PieceType::Bishop, Color::White);
        self.add_piece(0, 6, PieceType::Knight, Color::White);
        self.add_piece(0, 7, PieceType::Rook, Color::White);

        for i in 0..8 {
            self.add_piece(1, i, PieceType::Pawn, Color::White);
            self.add_piece(6, i, PieceType::Pawn, Color::Black);
        }
        
        self.add_piece(7, 0, PieceType::Rook, Color::Black);
        self.add_piece(7, 1, PieceType::Knight, Color::Black);
        self.add_piece(7, 2, PieceType::Bishop, Color::Black);
        self.add_piece(7, 3, PieceType::Queen, Color::Black);
        self.add_piece(7, 4, PieceType::King, Color::Black);
        self.add_piece(7, 5, PieceType::Bishop, Color::Black);
        self.add_piece(7, 6, PieceType::Knight, Color::Black);
        self.add_piece(7, 7, PieceType::Rook, Color::Black);
    }

    
}
