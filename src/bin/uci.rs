//! UCI (Universal Chess Interface) front-end for the engine in `chess::ai`.
//!
//! Chess GUIs and match tools (cutechess-cli, fastchess, Arena, lichess-bot) start this
//! program and talk to it in plain text over stdin/stdout, for example:
//!
//! ```text
//! > uci                                   < id name ... / option ... / uciok
//! > isready                               < readyok
//! > position startpos moves e2e4 e7e5
//! > go wtime 60000 btime 60000            < info depth 1 ... / ... / bestmove g1f3
//! ```
//!
//! The search runs on a background thread so "stop" and "isready" are answered while
//! it is thinking. Only UCI messages go to stdout; anything else would confuse the GUI.
//!
//! Run with `cargo run --release --bin uci`, or point a GUI at `target/release/uci`.

use std::io::{self, BufRead};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use chess::ai::board::{Board, Color, Move};
use chess::ai::search::{iterative_deepening, Search, SearchLimits, MATE_SCORE};

const ENGINE_NAME: &str = "chess_rust";
const ENGINE_AUTHOR: &str = "eddiechristian";
const MAX_DEPTH: u32 = 64;
/// Time kept back each move for communication delays, so the clock never runs out.
const MOVE_OVERHEAD: Duration = Duration::from_millis(30);

/// Settings a GUI can change with "setoption". Quiescence and move ordering can be
/// switched off to measure how much each is worth in engine-vs-engine matches.
struct Options {
    max_depth: u32,
    quiescence: bool,
    order_moves: bool,
}

/// A search running on a background thread.
struct RunningSearch {
    handle: JoinHandle<()>,
    stop: Arc<AtomicBool>,
    infinite: bool,
}

fn main() {
    let mut board = Board::start();
    let mut options = Options { max_depth: MAX_DEPTH, quiescence: true, order_moves: true };
    let mut running: Option<RunningSearch> = None;

    for line in io::stdin().lock().lines() {
        let Ok(line) = line else { break };
        let mut tokens = line.split_whitespace();
        let Some(command) = tokens.next() else { continue };

        match command {
            "uci" => {
                println!("id name {}", ENGINE_NAME);
                println!("id author {}", ENGINE_AUTHOR);
                println!("option name Depth type spin default {} min 1 max {}", MAX_DEPTH, MAX_DEPTH);
                println!("option name Quiescence type check default true");
                println!("option name MoveOrdering type check default true");
                println!("uciok");
            }
            "isready" => println!("readyok"),
            "setoption" => {
                finish(&mut running);
                set_option(&line, &mut options);
            }
            "ucinewgame" => {
                finish(&mut running);
                board = Board::start();
            }
            "position" => {
                finish(&mut running);
                match parse_position(&line) {
                    Ok(new_board) => board = new_board,
                    Err(e) => println!("info string {}", e),
                }
            }
            "go" => {
                finish(&mut running);
                running = Some(start_search(board, &line, &options));
            }
            "stop" => finish(&mut running),
            "quit" => {
                finish(&mut running);
                return;
            }
            // not part of UCI, but handy when testing by hand (Stockfish has it too)
            "d" => println!("{}", board),
            _ => {} // the UCI spec says to ignore unknown commands
        }
    }
    // end of input (e.g. commands piped in from a file): let a timed search finish
    // rather than cutting it short; an infinite one would never end, so stop it
    if let Some(search) = running.take() {
        if search.infinite {
            search.stop.store(true, Ordering::Relaxed);
        }
        let _ = search.handle.join();
    }
}

/// Stops a running search (if any) and waits for it to print its "bestmove".
fn finish(running: &mut Option<RunningSearch>) {
    if let Some(search) = running.take() {
        search.stop.store(true, Ordering::Relaxed);
        let _ = search.handle.join();
    }
}

/// "setoption name <name> value <value>"
fn set_option(line: &str, options: &mut Options) {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    let name_at = tokens.iter().position(|&t| t == "name");
    let value_at = tokens.iter().position(|&t| t == "value");
    let (Some(name_at), Some(value_at)) = (name_at, value_at) else { return };
    if value_at <= name_at {
        return;
    }
    let name = tokens[name_at + 1..value_at].join(" ").to_lowercase();
    let value = tokens.get(value_at + 1).copied().unwrap_or("");
    match name.as_str() {
        "depth" => {
            if let Ok(depth) = value.parse::<u32>() {
                options.max_depth = depth.clamp(1, MAX_DEPTH);
            }
        }
        "quiescence" => options.quiescence = value.eq_ignore_ascii_case("true"),
        "moveordering" => options.order_moves = value.eq_ignore_ascii_case("true"),
        _ => println!("info string unknown option {}", name),
    }
}

/// "position startpos [moves e2e4 ...]" or "position fen <fen> [moves e2e4 ...]"
fn parse_position(line: &str) -> Result<Board, String> {
    let tokens: Vec<&str> = line.split_whitespace().skip(1).collect();
    let moves_at = tokens.iter().position(|&t| t == "moves").unwrap_or(tokens.len());
    let (setup, moves) = (&tokens[..moves_at], tokens.get(moves_at + 1..).unwrap_or(&[]));

    let mut board = match setup.first() {
        Some(&"startpos") => Board::start(),
        Some(&"fen") => Board::from_fen(&setup[1..].join(" "))
            .ok_or_else(|| format!("invalid fen {}", setup[1..].join(" ")))?,
        _ => return Err("position needs startpos or fen".to_string()),
    };
    for text in moves {
        let mv = find_move(&board, text).ok_or_else(|| format!("illegal move {}", text))?;
        board = board.make_move(mv);
    }
    Ok(board)
}

/// The legal move written as `text` in UCI notation (e.g. e2e4, e7e8q, e1g1 for castling).
fn find_move(board: &Board, text: &str) -> Option<Move> {
    let text = text.to_lowercase();
    board.legal_moves().into_iter().find(|mv| mv.to_uci() == text)
}

/// Parses "go ..." into search limits.
/// - `depth N`: search exactly N deep
/// - `movetime MS`: think for MS milliseconds
/// - `wtime/btime/winc/binc/movestogo`: play on a clock, using a share of the time left
/// - `infinite`: think until "stop"
/// Returns the limits and whether the search is infinite.
fn parse_go(line: &str, side: Color, options: &Options) -> (SearchLimits, bool) {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    let number = |name: &str| -> Option<u64> {
        let i = tokens.iter().position(|&t| t == name)?;
        tokens.get(i + 1)?.parse().ok()
    };
    let infinite = tokens.contains(&"infinite");
    let now = Instant::now();

    let mut limits = SearchLimits {
        max_depth: number("depth").map_or(options.max_depth, |d| (d as u32).clamp(1, MAX_DEPTH)),
        ..SearchLimits::default()
    };
    if infinite {
        return (limits, true);
    }

    let (time_left, increment) = match side {
        Color::White => (number("wtime"), number("winc")),
        Color::Black => (number("btime"), number("binc")),
    };
    let budget = if let Some(movetime) = number("movetime") {
        Some(Duration::from_millis(movetime))
    } else if let Some(time_left) = time_left {
        // spread the remaining time over the moves still to play (assume 30 if unknown),
        // plus most of the increment, but never more than half the clock
        let moves_to_go = number("movestogo").unwrap_or(30).max(1);
        let share = time_left / moves_to_go + increment.unwrap_or(0) * 3 / 4;
        Some(Duration::from_millis(share.min(time_left / 2)))
    } else {
        None // no time given (e.g. "go depth 6"): just use the depth
    };

    if let Some(budget) = budget {
        let budget = budget.saturating_sub(MOVE_OVERHEAD).max(Duration::from_millis(1));
        limits.hard_deadline = Some(now + budget);
        // each depth takes several times longer than the last, so a depth started after
        // half the budget would rarely finish
        limits.soft_deadline = Some(now + budget / 2);
    }
    (limits, false)
}

/// Starts thinking on a background thread. It prints "info" after each depth and
/// "bestmove" at the end.
fn start_search(board: Board, line: &str, options: &Options) -> RunningSearch {
    let (mut limits, infinite) = parse_go(line, board.side_to_move, options);
    let stop = Arc::new(AtomicBool::new(false));
    limits.stop_signal = Some(stop.clone());
    let settings = Search {
        order_moves: options.order_moves,
        quiescence: options.quiescence,
        ..Search::default()
    };
    let stop_for_thread = stop.clone();

    let handle = thread::spawn(move || {
        let start = Instant::now();
        let result = iterative_deepening(&board, &limits, settings, |depth, r| {
            let ms = start.elapsed().as_millis().max(1);
            println!(
                "info depth {} score {} nodes {} time {} nps {} pv {}",
                depth,
                uci_score(r.score, board.side_to_move, depth),
                r.nodes,
                ms,
                r.nodes as u128 * 1000 / ms,
                r.best.to_uci()
            );
        });
        // with "go infinite" the GUI expects bestmove only after it sends "stop"
        if infinite {
            while !stop_for_thread.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(5));
            }
        }
        match result {
            Some(r) => println!("bestmove {}", r.best.to_uci()),
            None => println!("bestmove 0000"), // no legal moves
        }
    });
    RunningSearch { handle, stop, infinite }
}

/// UCI scores are from the side to move's point of view: "cp N" in centipawns, or
/// "mate N" (N moves to mate; negative if the engine is being mated). The engine's
/// scores are from white's point of view, so they are flipped for black.
fn uci_score(score: i32, side: Color, depth: u32) -> String {
    let relative = if side == Color::White { score } else { -score };
    if relative.abs() >= MATE_SCORE {
        // mate scores are MATE_SCORE + depth left when mate happened, which gives the
        // number of half-moves from here (approximate if the mate came in quiescence)
        let plies = (depth as i32 - (relative.abs() - MATE_SCORE)).max(1);
        let moves = (plies + 1) / 2;
        if relative > 0 { format!("mate {}", moves) } else { format!("mate -{}", moves) }
    } else {
        format!("cp {}", relative)
    }
}
