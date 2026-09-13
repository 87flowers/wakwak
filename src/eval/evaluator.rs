use super::piece_square_tables::{EG_PIECE_SQUARE_TABLES, MG_PIECE_SQUARE_TABLES};
use crate::board::Board;
use crate::common::Color;
use crate::common::Piece::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::score::Score;

const MG_PIECE_VALUES: [i32; 6] = [82, 337, 365, 477, 1025, 0];
const EG_PIECE_VALUES: [i32; 6] = [94, 281, 297, 512, 936, 0];
const MG_PIECE_SCORES: [[i32; 64]; 6] = combine_scores(MG_PIECE_VALUES, MG_PIECE_SQUARE_TABLES);
const EG_PIECE_SCORES: [[i32; 64]; 6] = combine_scores(EG_PIECE_VALUES, EG_PIECE_SQUARE_TABLES);
const PHASE_WEIGHTS: [i32; 6] = [0, 1, 1, 2, 4, 0];
const MAX_PHASE: i32 = 24;

const fn combine_scores(values: [i32; 6], mut tables: [[i32; 64]; 6]) -> [[i32; 64]; 6] {
    let mut piece = 0;
    while piece < 6 {
        let mut square = 0;
        while square < 64 {
            tables[piece][square] += values[piece];
            square += 1;
        }
        piece += 1;
    }
    tables
}

pub fn evaluate(board: &Board) -> Score {
    let us = board.stm();
    let them = !us;
    let (us_mg, us_eg, us_phase) = side_score(board, us);
    let (them_mg, them_eg, them_phase) = side_score(board, them);
    let phase = (us_phase + them_phase).min(MAX_PHASE);
    ((us_mg - them_mg) * phase + (us_eg - them_eg) * (MAX_PHASE - phase)) / MAX_PHASE
}

fn side_score(board: &Board, color: Color) -> (Score, Score, i32) {
    let mut mg = Score::ZERO;
    let mut eg = Score::ZERO;
    let mut phase = 0;
    for piece in [Pawn, Knight, Bishop, Rook, Queen, King] {
        for square in board.colored_pieces(color, piece) {
            let square = square.relative_to(color);
            mg += MG_PIECE_SCORES[piece][square];
            eg += EG_PIECE_SCORES[piece][square];
            phase += PHASE_WEIGHTS[piece];
        }
    }
    (mg, eg, phase)
}
