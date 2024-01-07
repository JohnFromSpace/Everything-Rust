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
    
    fn add_piece(&mut self, row: usize, col: usize, piece_type: PieceType, color: Color) {
        self.squares[row][col] = Some(Piece {
            color,
            piece_type,
        });
    }

    fn print(&self) {
        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = self.squares[i][j] {
                    let piece_str = match piece.piece_type {
                        PieceType::Pawn => 'P',
                        PieceType::Rook => 'R',
                        PieceType::Knight => 'N',
                        PieceType::Bishop => 'B',
                        PieceType::Queen => 'Q',
                        PieceType::King => 'K',
                    };
                    let color_str = match piece.color {
                        Color::White => 'W',
                        Color::Black => 'B',
                    };
                    print!("{}{} ", color_str, piece_str);
                } else {
                    print!("-- ");
                }
            }
            println!();
        }
    }

     fn is_valid_move(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        let (from_row, from_col) = from;
        let (to_row, to_col) = to;

        if from_row >= 8 || from_col >= 8 || to_row >= 8 || to_col >= 8 {
            return false; // Out of bounds
        }

        if let Some(piece) = self.squares[from_row][from_col] {
            if piece.color != self.turn {
                return false; // Wrong player's turn
            }

            match piece.piece_type {
                PieceType::Pawn => self.is_valid_pawn_move(from, to),
                PieceType::Rook => self.is_valid_rook_move(from, to),
                PieceType::Knight => self.is_valid_knight_move(from, to),
                PieceType::Bishop => self.is_valid_bishop_move(from, to),
                PieceType::Queen => self.is_valid_queen_move(from, to),
                PieceType::King => self.is_valid_king_move(from, to),
            }
        } else {
            false // No piece at the starting square
        }
    }
    
    fn is_valid_pawn_move(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        // Implement pawn move validation logic
        // This is a simplified example and does not cover all pawn rules
        let (from_row, from_col) = from;
        let (to_row, to_col) = to;

        let direction = if self.turn == Color::White { 1 } else { -1 };

        if to_col == from_col && self.squares[to_row][to_col].is_none() {
            if to_row == from_row + direction {
                return true;
            }
            if from_row == 1 && to_row == from_row + 2 * direction && self.squares[from_row + direction][from_col].is_none() {
                return true;
            }
        }

        false
    }

    fn is_valid_rook_move(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        // Implement rook move validation logic
        let (from_row, from_col) = from;
        let (to_row, to_col) = to;

        if from_row == to_row || from_col == to_col {
            // Rook moves horizontally or vertically
            return true;
        }

        false
    }

    fn is_valid_knight_move(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        // Implement knight move validation logic
        let (from_row, from_col) = from;
        let (to_row, to_col) = to;

        let row_diff = (to_row as isize - from_row as isize).abs();
        let col_diff = (to_col as isize - from_col as isize).abs();

        (row_diff == 2 && col_diff == 1) || (row_diff == 1 && col_diff == 2)
    }

    fn is_valid_bishop_move(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        // Implement bishop move validation logic
        let (from_row, from_col) = from;
        let (to_row, to_col) = to;

        if (to_row as isize - from_row as isize).abs() == (to_col as isize - from_col as isize).abs() {
            // Bishop moves diagonally
            return true;
        }

        false
    }

    fn is_valid_queen_move(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        // Implement queen move validation logic
        self.is_valid_rook_move(from, to) || self.is_valid_bishop_move(from, to)
    }

    fn is_valid_king_move(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        // Implement king move validation logic
        let (from_row, from_col) = from;
        let (to_row, to_col) = to;

        let row_diff = (to_row as isize - from_row as isize).abs();
        let col_diff = (to_col as isize - from_col as isize).abs();

        row_diff <= 1 && col_diff <= 1
    }

    fn perform_move(&mut self, from: (usize, usize), to: (usize, usize)) {
        // Execute the move
        self.squares[to.0][to.1] = self.squares[from.0][from.1];
        self.squares[from.0][from.1] = None;

        // Switch player's turn
        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }

    fn is_checkmate(&self) -> bool {
        // Implement checkmate detection logic
        // This is a simplified example and does not cover all checkmate conditions
        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = self.squares[i][j] {
                    if piece.color == self.turn {
                        for x in 0..8 {
                            for y in 0..8 {
                                let from = (i, j);
                                let to = (x, y);

                                if self.is_valid_move(from, to) && !self.leads_to_check(from, to) {
                                    return false;
                                }
                            }
                        }
                    }
                }
            }
        }

        true
    }

    fn leads_to_check(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        let mut board_copy = self.clone();
        board_copy.perform_move(from, to);
        board_copy.is_check()
    }

    fn is_check(&self) -> bool {
        // Implement check detection logic
        // This is a simplified example and does not cover all check conditions
        let king_position = self.find_king(self.turn);

        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = self.squares[i][j] {
                    if piece.color != self.turn {
                        let from = (i, j);
                        let to = king_position;

                        if self.is_valid_move(from, to) {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    fn find_king(&self, color: Color) -> (usize, usize) {
        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = self.squares[i][j] {
                    if piece.piece_type == PieceType::King && piece.color == color {
                        return (i, j);
                    }
                }
            }
        }
        unreachable!()
    }
}

fn main() {
    let mut board = Board::new();
    board.init();
    board.print();     

    loop {
        println!("Enter the move (from_row from_col to_row to_col):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let mut iter = input.split_whitespace();
        let from = (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        );
        
        let to = (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        );

        if !board.is_valid_move(from, to) {
            println!("Invalid move. Try again.");
            continue;
        }
    }
}
