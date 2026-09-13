pub mod quiet;

use crate::board::Board;
use crate::common::Move;
pub use quiet::*;

pub const MAX_HISTORY: i32 = 16384;

pub struct History {
    quiet: QuietHistory,
}

impl History {
    #[inline]
    pub fn quiet(&self, board: &Board, mv: Move) -> i32 {
        self.quiet.entry(board, mv)
    }

    #[inline]
    pub fn update(&mut self, board: &Board, depth: i32, best_move: Move, failed_quiets: &[Move]) {
        if best_move.flag().is_quiet() {
            self.update_quiet::<true>(board, depth, best_move);
            for &quiet in failed_quiets {
                self.update_quiet::<false>(board, depth, quiet);
            }
        }
    }

    #[inline]
    fn update_quiet<const BONUS: bool>(&mut self, board: &Board, depth: i32, mv: Move) {
        self.quiet.update::<BONUS>(board, depth, mv);
    }
}

#[inline]
pub fn gravity_with_decay<const MAX_BONUS: i32, const MAX_VALUE: i32>(
    entry: &mut i16,
    decay: i32,
    amount: i32,
) {
    let amount = amount.clamp(-MAX_BONUS, MAX_BONUS);
    let decay = (decay * amount.abs() / MAX_VALUE) as i16;
    *entry += amount as i16 - decay;
}

#[inline]
pub fn gravity<const MAX_BONUS: i32, const MAX_VALUE: i32>(entry: &mut i16, amount: i32) {
    gravity_with_decay::<MAX_BONUS, MAX_VALUE>(entry, *entry as i32, amount);
}
