use crate::board::Board;
use crate::common::Color;
use crate::common::Piece::{Bishop, King, Knight, Pawn, Queen, Rook};
use super::piece_square_tables::PIECE_SQUARE_TABLES;

const PIECE_VALUES: [i32; 6] = [100, 320, 330, 500, 900, 0];

pub fn evaluate(board: &Board) -> i32 {
    let us = board.stm();
    let them = !us;
    material_score(board, us) - material_score(board, them)
        + psq_score(board, us) - psq_score(board, them)
}

fn material_score(board: &Board, color: Color) -> i32 {
    let mut score = 0;
    for piece in [Pawn, Knight, Bishop, Rook, Queen] {
        let count = board.colored_pieces(color, piece).popcnt();
        score += PIECE_VALUES[piece] * count as i32;
    }
    score
}

fn psq_score(board: &Board, color: Color) -> i32 {
    let mut score = 0;
    for piece in [Pawn, Knight, Bishop, Rook, Queen, King] {
        for square in board.colored_pieces(color, piece) {
            score += PIECE_SQUARE_TABLES[piece][square.relative_to(color)];
        }
    }
    score
}
