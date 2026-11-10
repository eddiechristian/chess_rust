//! Chess AI.
//! - `board`: compact, cheap-to-copy board and legal move generation
//! - `eval`: scores a position (material + piece-square tables)
//! - `search`: minimax / alpha-beta, picks the best move

pub mod board;
pub mod eval;
pub mod search;
