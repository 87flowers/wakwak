use crate::board::Board;
use crate::common::{Move, MoveFlag, Piece};
use crate::position::{Position};
use crate::search::MAX_PLY;
use crate::util::Abort;

pub struct ScoredMove(Move, i32);

// Indexed by PieceType as usize (Pawn, Knight, Bishop, Rook, Queen, King)
const PIECE_VALUE: [i32; 6] = [100, 320, 330, 500, 900, 20000];

pub struct MoveStack {
    stack: Vec<ScoredMove>,
    start: [usize; MAX_PLY + 1],
    ply: usize,
}

impl MoveStack {
    #[inline]
    pub fn push(&mut self, board: &Board) {
        debug_assert!(
            self.ply < MAX_PLY,
            "MoveStack::push(): Attempted to push on ply `MAX_PLY`"
        );

        self.stack.truncate(self.start[self.ply]);

        let mut cursor = self.start[self.ply];
        board.gen_moves(|moves| {
            self.stack.extend(moves.iter().map(|w| ScoredMove(w, 0)));
            cursor += moves.len();
            Abort::No
        });

        self.start[self.ply + 1] = cursor;
        self.ply += 1;
    }

    #[inline]
    pub fn pop(&mut self) {
        debug_assert!(self.ply > 0, "MoveStack::pop(): Empty stack");

        self.ply -= 1;
        self.stack.truncate(self.start[self.ply]);
    }

    #[inline]
    pub fn get(&self) -> &[ScoredMove] {
        debug_assert!(self.ply > 0, "MoveStack::get(): Empty stack");

        &self.stack[self.start[self.ply - 1]..]
    }

    #[inline]
    pub fn get_mut(&mut self) -> &mut [ScoredMove] {
        debug_assert!(self.ply > 0, "MoveStack::get_mut(): Empty stack");

        &mut self.stack[self.start[self.ply - 1]..]
    }

    #[inline]
    pub fn reset(&mut self) {
        self.stack.clear();
        self.ply = 0;
    }
}

impl Default for MoveStack {
    #[inline]
    fn default() -> Self {
        Self {
            stack: Vec::new(),
            start: [0; MAX_PLY + 1],
            ply: 0,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stage {
    SplitNoisy,
    YieldNoisy,
    YieldQuiet,
    Finished,
}

pub struct MovePicker {
    stage: Stage,
    skip_quiets: bool,
    noisy_count: usize,
    cursor: usize,
}

impl MovePicker {
    #[inline]
    pub fn skip_quiets(&mut self) {
        self.skip_quiets = true;
        if matches!(self.stage, Stage::YieldQuiet) {
            self.stage = Stage::Finished;
        }
    }

    pub fn next(&mut self, position: &Position, moves: &mut [ScoredMove]) -> Option<Move> {
        let board = position.board();
        if self.stage == Stage::SplitNoisy {
            // Move all noisies to the front of the list
            let mut i = 0;
            for j in 0..moves.len() {
                let mv = moves[j].0;
                if moves[j].0.flag().is_noisy() {
                    // Score noisies here (moves[j].1 = pluh)
                    let attacker = mv.flag().promotion().or(board.piece_on(mv.src())).unwrap();
                    let attacker_value = PIECE_VALUE[attacker as usize];
                    let victim_value = if mv.flag() == MoveFlag::EnPassant {
                        PIECE_VALUE[Piece::Pawn as usize]
                    } else if mv.flag().is_capture() {
                        PIECE_VALUE[board.piece_on(mv.dest()).unwrap() as usize]
                    } else {
                        0
                    };
                    moves[j].1 = victim_value - attacker_value;
                    moves.swap(i, j);
                    i += 1;
                } else {
                    // Score quiets here (moves[j].1 = pluh)
                }
            }

            self.noisy_count = i;
            self.stage = Stage::YieldNoisy;
        }

        if self.stage == Stage::YieldNoisy {
            if self.skip_quiets {
                self.stage = Stage::Finished;
            } else if self.cursor >= self.noisy_count {
                self.stage = Stage::YieldQuiet;
            } else {
                let (i, mv) = self.select_next(&moves[..self.noisy_count]);
                println!("{} -> score {}", mv.display(false, false), moves[i].1);
                moves.swap(self.cursor, i);
                self.cursor += 1;

                return Some(mv);
            }
        }

        if self.stage == Stage::YieldQuiet {
            if self.skip_quiets {
                // Not sure if it's possible to hit this branch but just to be sure
                self.stage = Stage::Finished;
            } else {
                if self.cursor < self.noisy_count {
                    self.cursor = self.noisy_count;
                }

                if self.cursor >= moves.len() {
                    self.stage = Stage::Finished;
                } else {
                    let (i, mv) = self.select_next(moves);
                    moves.swap(self.cursor, i);
                    self.cursor += 1;

                    return Some(mv);
                }
            }
        }

        None
    }

    #[inline]
    fn select_next(&self, moves: &[ScoredMove]) -> (usize, Move) {
        let i = moves
            .iter()
            .enumerate()
            .skip(self.cursor)
            .max_by_key(|(_, mv)| mv.1)
            .map(|(i, _)| i)
            .unwrap();

        (i, moves[i].0)
    }
}

impl Default for MovePicker {
    #[inline]
    fn default() -> Self {
        Self {
            stage: Stage::SplitNoisy,
            skip_quiets: false,
            noisy_count: 0,
            cursor: 0,
        }
    }
}

#[test]
fn debug_print_scores() {
    let board = Board::from_fen("r3k3/1P6/8/1q1pP3/2P5/8/P4PPP/4K3 w - d6 0 1").unwrap();
    let pos = Position::new(board);
    let mut move_stack = MoveStack::default();
    move_stack.push(pos.board());

    let mut picker = MovePicker::default();
    while let Some(_) = picker.next(&pos, move_stack.get_mut()) {
    }
    assert!(false)
}