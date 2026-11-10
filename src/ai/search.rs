//! Minimax and alpha-beta search. Positions are scored by `eval::evaluate`
//! (material plus piece-square tables).
//!
//! Scores are in centipawns (100 = one pawn) from white's point of view:
//! positive is good for white, negative is good for black.
//! White picks the highest-scoring move and black the lowest.
//!
//! Alpha-beta returns exactly the same score as minimax; it only skips branches
//! that can't change the result. `Search::nodes` counts positions visited so the
//! two can be compared.
//!
//! Quiescence search: when the depth runs out, captures are played out until the
//! position is quiet before it is scored. Without it, the search can stop halfway
//! through an exchange (e.g. right after QxP, before the recapture) and badly misjudge
//! the position: the "horizon effect".

use super::board::{Board, Color, Kind, Move};
pub use super::eval::{evaluate, piece_value};

/// Score for being checkmated. Far bigger than any material difference.
pub const MATE_SCORE: i32 = 100_000;

/// Score when the side to move has no legal moves: checkmate or stalemate.
/// Adding `depth` makes a quicker mate score higher, so the winning side goes for the
/// fastest mate and the losing side delays it.
fn game_over_score(board: &Board, depth: u32) -> i32 {
    if board.in_check(board.side_to_move) {
        let score = MATE_SCORE + depth as i32;
        match board.side_to_move {
            Color::White => -score,
            Color::Black => score,
        }
    } else {
        0 // stalemate is a draw
    }
}

/// Sorts captures first, best captures first: most valuable victim, then least valuable
/// attacker (pawn takes queen before queen takes pawn). Promotions count as captures of
/// the promoted piece's value. Quiet moves keep their original order at the end.
/// Alpha-beta cuts off most when the best move is searched first.
fn order_moves(board: &Board, moves: &mut [Move]) {
    let priority = |mv: &Move| -> i32 {
        let attacker = board.squares[mv.from as usize].map_or(Kind::Pawn, |p| p.kind);
        let victim = match board.squares[mv.to as usize] {
            Some(p) => Some(p.kind),
            // en passant: the destination is empty but a pawn is captured
            None if attacker == Kind::Pawn && Some(mv.to) == board.en_passant => Some(Kind::Pawn),
            None => None,
        };
        let mut score = 0;
        if let Some(victim) = victim {
            score += 10 * piece_value(victim) - piece_value(attacker) + 10_000;
        }
        if let Some(promotion) = mv.promotion {
            score += piece_value(promotion);
        }
        score
    };
    // stable sort, highest priority first
    moves.sort_by_key(|mv| std::cmp::Reverse(priority(mv)));
}

/// Captures (including en passant) and promotions: the moves that change material,
/// which quiescence search keeps playing out.
fn is_noisy(board: &Board, mv: &Move) -> bool {
    let en_passant = Some(mv.to) == board.en_passant
        && board.squares[mv.from as usize].map_or(false, |p| p.kind == Kind::Pawn);
    board.squares[mv.to as usize].is_some() || en_passant || mv.promotion.is_some()
}

/// A search in progress: counts the positions it visits.
pub struct Search {
    /// positions visited so far
    pub nodes: u64,
    /// search captures first in alpha-beta (no effect on plain minimax)
    pub order_moves: bool,
    /// play out captures at the end of the search before scoring (quiescence search)
    pub quiescence: bool,
}

impl Default for Search {
    fn default() -> Self {
        Search { nodes: 0, order_moves: true, quiescence: true }
    }
}

impl Search {
    pub fn new() -> Self {
        Search::default()
    }

    /// Plain minimax: looks `depth` moves ahead (counting both sides' moves) and returns
    /// the score assuming both sides play their best. Searches every branch.
    pub fn minimax(&mut self, board: &Board, depth: u32) -> i32 {
        if depth == 0 {
            return self.leaf(board, i32::MIN, i32::MAX);
        }
        self.nodes += 1;
        let moves = board.legal_moves();
        if moves.is_empty() {
            return game_over_score(board, depth);
        }
        let scores = moves.iter().map(|&mv| self.minimax(&board.make_move(mv), depth - 1));
        match board.side_to_move {
            Color::White => scores.max().unwrap(),
            Color::Black => scores.min().unwrap(),
        }
    }

    /// Alpha-beta: the same score as minimax, but skips branches that can't matter.
    /// - `alpha`: the score white is already guaranteed elsewhere; white won't accept less.
    /// - `beta`: the score black is already guaranteed elsewhere; black won't allow more.
    /// Once `alpha >= beta` the side choosing above would never let play reach this
    /// position, so the remaining moves here are skipped (a "cutoff").
    /// Call with `i32::MIN, i32::MAX` for an exact score.
    pub fn alphabeta(&mut self, board: &Board, depth: u32, mut alpha: i32, mut beta: i32) -> i32 {
        if depth == 0 {
            return self.leaf(board, alpha, beta);
        }
        self.nodes += 1;
        let mut moves = board.legal_moves();
        if moves.is_empty() {
            return game_over_score(board, depth);
        }
        if self.order_moves {
            order_moves(board, &mut moves);
        }

        match board.side_to_move {
            Color::White => {
                let mut best = i32::MIN;
                for mv in moves {
                    let score = self.alphabeta(&board.make_move(mv), depth - 1, alpha, beta);
                    best = best.max(score);
                    alpha = alpha.max(score); // white's guarantee improves
                    if alpha >= beta {
                        break; // black already has something better elsewhere: cut
                    }
                }
                best
            }
            Color::Black => {
                let mut best = i32::MAX;
                for mv in moves {
                    let score = self.alphabeta(&board.make_move(mv), depth - 1, alpha, beta);
                    best = best.min(score);
                    beta = beta.min(score); // black's guarantee improves
                    if alpha >= beta {
                        break; // white already has something better elsewhere: cut
                    }
                }
                best
            }
        }
    }

    /// Scores a position where the depth has run out: with quiescence search on, plays
    /// out captures first; otherwise just evaluates it.
    fn leaf(&mut self, board: &Board, alpha: i32, beta: i32) -> i32 {
        if self.quiescence {
            return self.quiesce(board, alpha, beta);
        }
        self.nodes += 1;
        if board.legal_moves().is_empty() {
            return game_over_score(board, 0);
        }
        evaluate(board)
    }

    /// Quiescence search: keeps searching captures and promotions (no depth limit) until
    /// the position is quiet, using the same alpha-beta window.
    ///
    /// "Standing pat": the side to move doesn't have to capture, so the static evaluation
    /// is the worst it will accept; it only captures if that does better. The exception
    /// is being in check, where every legal move is searched because the check must be
    /// answered. This always ends: every capture removes a piece, and check evasions are
    /// only searched straight after a move that gave check.
    pub fn quiesce(&mut self, board: &Board, mut alpha: i32, mut beta: i32) -> i32 {
        self.nodes += 1;
        let mut moves = board.legal_moves();
        if moves.is_empty() {
            return game_over_score(board, 0);
        }
        let in_check = board.in_check(board.side_to_move);
        if !in_check {
            moves.retain(|mv| is_noisy(board, mv));
        }
        order_moves(board, &mut moves);

        let white = board.side_to_move == Color::White;
        // standing pat: score if we stop capturing now (not allowed while in check)
        let mut best = if in_check {
            if white { i32::MIN } else { i32::MAX }
        } else {
            let stand_pat = evaluate(board);
            if white {
                if stand_pat >= beta {
                    return stand_pat; // already too good: black won't allow this position
                }
                alpha = alpha.max(stand_pat);
            } else {
                if stand_pat <= alpha {
                    return stand_pat; // already too good for black: white won't allow it
                }
                beta = beta.min(stand_pat);
            }
            stand_pat
        };

        for mv in moves {
            let score = self.quiesce(&board.make_move(mv), alpha, beta);
            if white {
                best = best.max(score);
                alpha = alpha.max(score);
            } else {
                best = best.min(score);
                beta = beta.min(score);
            }
            if alpha >= beta {
                break;
            }
        }
        best
    }
}

/// The move chosen by a search, its score, and how many positions were visited.
#[derive(Debug, Clone, Copy)]
pub struct SearchResult {
    pub best: Move,
    pub score: i32,
    pub nodes: u64,
}

/// The best move for the side to move using alpha-beta with move ordering.
/// Returns None if there are no legal moves (checkmate or stalemate).
/// When several moves score the same, the first one searched is chosen.
pub fn best_move(board: &Board, depth: u32) -> Option<SearchResult> {
    best_move_with(board, depth, Search::new(), true)
}

/// The best move using plain minimax, for comparing against `best_move`.
pub fn best_move_minimax(board: &Board, depth: u32) -> Option<SearchResult> {
    best_move_with(board, depth, Search::new(), false)
}

/// Searches each root move. With `use_alphabeta`, the root also narrows the window as
/// better moves are found, so later root moves can be cut off early.
pub fn best_move_with(
    board: &Board,
    depth: u32,
    mut search: Search,
    use_alphabeta: bool,
) -> Option<SearchResult> {
    search.nodes += 1; // the root position
    let mut moves = board.legal_moves();
    if use_alphabeta && search.order_moves {
        order_moves(board, &mut moves);
    }
    let white = board.side_to_move == Color::White;
    let (mut alpha, mut beta) = (i32::MIN, i32::MAX);
    let mut best: Option<(Move, i32)> = None;

    for mv in moves {
        let next = board.make_move(mv);
        let child_depth = depth.saturating_sub(1);
        let score = if use_alphabeta {
            search.alphabeta(&next, child_depth, alpha, beta)
        } else {
            search.minimax(&next, child_depth)
        };
        // strictly better only: a cut-off branch can return a score equal to the current
        // best without really being as good, so ties keep the earlier move
        let better = match best {
            None => true,
            Some((_, best_score)) => if white { score > best_score } else { score < best_score },
        };
        if better {
            best = Some((mv, score));
            if white { alpha = score } else { beta = score }
        }
    }

    best.map(|(best, score)| SearchResult { best, score, nodes: search.nodes })
}

#[cfg(test)]
mod tests {
    use super::*;

    const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    const POSITION_3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
    const POSITION_4: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
    const MATE_IN_ONE_WHITE: &str = "6k1/5ppp/8/8/8/8/8/R5K1 w - - 0 1";
    const MATE_IN_ONE_BLACK: &str = "r5k1/8/8/8/8/8/5PPP/6K1 b - - 0 1";
    /// white queen d1 can take the d5 pawn, but the e6 pawn recaptures
    const DEFENDED_PAWN: &str = "4k3/8/4p3/3p4/8/8/8/3QK3 w - - 0 1";

    fn board(fen: &str) -> Board {
        Board::from_fen(fen).expect("valid FEN")
    }

    fn search(order_moves: bool, quiescence: bool) -> Search {
        Search { nodes: 0, order_moves, quiescence }
    }

    fn best(fen: &str, depth: u32) -> (String, i32) {
        let result = best_move(&board(fen), depth).expect("has a legal move");
        (result.best.to_notation(), result.score)
    }

    fn best_without_quiescence(fen: &str, depth: u32) -> String {
        best_move_with(&board(fen), depth, search(true, false), true)
            .expect("has a legal move")
            .best
            .to_notation()
    }

    /// The key property: alpha-beta (with or without ordering) gives exactly the
    /// same score as minimax. It is only allowed to be faster. This holds with
    /// quiescence search on or off, as long as both searches use the same setting.
    #[test]
    fn alphabeta_matches_minimax() {
        // (position, max depth without quiescence, max depth with quiescence)
        let cases = [
            // (plain minimax with quiescence plays out every capture sequence with no
            // pruning, so it is very slow; keep its depths small)
            (crate::ai::board::START_FEN, 4, 2),
            (KIWIPETE, 3, 1),
            (POSITION_3, 4, 2),
            (POSITION_4, 3, 1),
            (MATE_IN_ONE_WHITE, 3, 2),
            (MATE_IN_ONE_BLACK, 3, 2),
            (DEFENDED_PAWN, 4, 2),
        ];
        for (fen, plain_depth, quiet_depth) in cases {
            let b = board(fen);
            for (quiescence, max_depth) in [(false, plain_depth), (true, quiet_depth)] {
                for depth in 1..=max_depth {
                    let expected = search(false, quiescence).minimax(&b, depth);
                    for order_moves in [false, true] {
                        let score = search(order_moves, quiescence).alphabeta(&b, depth, i32::MIN, i32::MAX);
                        assert_eq!(
                            score, expected,
                            "fen {} depth {} ordered {} quiescence {}",
                            fen, depth, order_moves, quiescence
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn best_move_scores_match_minimax() {
        for fen in [KIWIPETE, POSITION_4, DEFENDED_PAWN] {
            let b = board(fen);
            // without quiescence, so plain minimax stays fast enough to compare
            let ab = best_move_with(&b, 3, search(true, false), true).unwrap();
            let mm = best_move_with(&b, 3, search(false, false), false).unwrap();
            assert_eq!(ab.score, mm.score, "fen {}", fen);
            assert!(ab.nodes < mm.nodes, "alpha-beta should visit fewer positions");
        }
    }

    #[test]
    fn white_finds_mate_in_one() {
        let (mv, score) = best(MATE_IN_ONE_WHITE, 1);
        assert_eq!(mv, "a1-a8");
        assert!(score >= MATE_SCORE);
    }

    #[test]
    fn black_finds_mate_in_one() {
        let (mv, score) = best(MATE_IN_ONE_BLACK, 1);
        assert_eq!(mv, "a8-a1");
        assert!(score <= -MATE_SCORE);
    }

    #[test]
    fn takes_a_free_queen() {
        let (mv, _) = best("4k3/8/8/3q4/8/8/8/3QK3 w - - 0 1", 2);
        assert_eq!(mv, "d1-d5");
    }

    /// The horizon effect, and quiescence search fixing it.
    #[test]
    fn quiescence_sees_the_recapture() {
        // without quiescence, looking 1 move ahead, taking the pawn looks like +1 pawn
        assert_eq!(best_without_quiescence(DEFENDED_PAWN, 1), "d1-d5");
        // quiescence plays out exd5 after the capture, even at depth 1
        assert_ne!(best(DEFENDED_PAWN, 1).0, "d1-d5");
        assert_ne!(best(DEFENDED_PAWN, 2).0, "d1-d5");
    }

    #[test]
    fn quiet_position_scores_as_evaluation() {
        // no captures available, so quiescence just "stands pat"
        let b = Board::start();
        assert_eq!(Search::new().quiesce(&b, i32::MIN, i32::MAX), evaluate(&b));
    }

    #[test]
    fn quiescence_wins_a_hanging_piece() {
        // black to move would recapture nothing: the d5 queen is simply hanging to e4xd5
        let b = board("4k3/8/8/3q4/4P3/8/8/4K3 w - - 0 1");
        let score = Search::new().quiesce(&b, i32::MIN, i32::MAX);
        assert!(score > evaluate(&b) + 700, "should count winning the queen");
    }

    #[test]
    fn stalemate_scores_zero() {
        // black to move: king on h8 has no moves and is not in check
        let b = board("7k/5Q2/6K1/8/8/8/8/8 b - - 0 1");
        assert!(b.legal_moves().is_empty());
        assert_eq!(Search::new().minimax(&b, 3), 0);
        assert_eq!(Search::new().alphabeta(&b, 3, i32::MIN, i32::MAX), 0);
        assert_eq!(Search::new().quiesce(&b, i32::MIN, i32::MAX), 0);
    }

    #[test]
    fn captures_are_ordered_first() {
        // white queen d1 can take the d5 queen; pawn e4 can take it too (better: pawn takes queen)
        let b = board("4k3/8/8/3q4/4P3/8/8/3QK3 w - - 0 1");
        let mut moves = b.legal_moves();
        order_moves(&b, &mut moves);
        assert_eq!(moves[0].to_notation(), "e4-d5");
        assert_eq!(moves[1].to_notation(), "d1-d5");
    }

    /// Prints positions visited by each search. Run with:
    /// cargo test --release compare_node_counts -- --ignored --nocapture
    #[test]
    #[ignore]
    fn compare_node_counts() {
        for (name, fen, max_depth) in [
            ("start", crate::ai::board::START_FEN, 5),
            ("kiwipete", KIWIPETE, 4),
        ] {
            println!("\n{} (first three columns without quiescence)", name);
            println!(
                "depth  minimax nodes   alpha-beta   alpha-beta + ordering   + quiescence   time (mm / ab+ord / +quiescence)"
            );
            let b = board(fen);
            for depth in 1..=max_depth {
                let t = std::time::Instant::now();
                let mm = best_move_with(&b, depth, search(false, false), false).unwrap();
                let mm_time = t.elapsed();
                let unordered = best_move_with(&b, depth, search(false, false), true).unwrap();
                let t = std::time::Instant::now();
                let ordered = best_move_with(&b, depth, search(true, false), true).unwrap();
                let ab_time = t.elapsed();
                let t = std::time::Instant::now();
                let quiet = best_move(&b, depth).unwrap();
                let q_time = t.elapsed();
                assert_eq!(mm.score, unordered.score);
                assert_eq!(mm.score, ordered.score);
                println!(
                    "{:>5}  {:>13}   {:>10}   {:>21}   {:>12}   {:.3}s / {:.3}s / {:.3}s",
                    depth, mm.nodes, unordered.nodes, ordered.nodes, quiet.nodes,
                    mm_time.as_secs_f64(), ab_time.as_secs_f64(), q_time.as_secs_f64()
                );
            }
        }
    }
}
