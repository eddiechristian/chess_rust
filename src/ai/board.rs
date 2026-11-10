//! Compact board for the AI.
//!
//! The whole board is a plain `Copy` value, so trying a move is just
//! `let next = board.make_move(mv);`. There is nothing to undo: the old board is unchanged.
//!
//! Square numbers match `GameState`: 0 = a8, 7 = h8, 56 = a1, 63 = h1
//! (row 0 is rank 8, row 7 is rank 1).
//!
//! Validation happens when moves are generated: `legal_moves` only returns legal moves,
//! and `make_move` trusts its input. Only call `make_move` with a move from `legal_moves`.

use std::fmt;

use crate::chess_notation_utilities;
use crate::visual::{self, GameState, PLAYER};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opponent(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Piece {
    pub color: Color,
    pub kind: Kind,
}

/// A move is two square numbers, plus the piece a pawn promotes to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub promotion: Option<Kind>,
}

impl Move {
    /// UCI notation used by chess engines and GUIs: `e2e4`, promotions as `e7e8q`.
    pub fn to_uci(&self) -> String {
        let mut text = format!(
            "{}{}",
            chess_notation_utilities::index_to_spot(self.from as usize),
            chess_notation_utilities::index_to_spot(self.to as usize)
        );
        if let Some(kind) = self.promotion {
            text.push(kind_letter(kind));
        }
        text
    }

    /// Long algebraic notation accepted by `Game::move_piece`, e.g. `e2-e4` or `e7-e8=Q`.
    pub fn to_notation(&self) -> String {
        let mut text = format!(
            "{}-{}",
            chess_notation_utilities::index_to_spot(self.from as usize),
            chess_notation_utilities::index_to_spot(self.to as usize)
        );
        if let Some(kind) = self.promotion {
            text.push('=');
            text.push(kind_letter(kind).to_ascii_uppercase());
        }
        text
    }
}

/// Indexes into `Board::castling`.
pub const WHITE_KINGSIDE: usize = 0;
pub const WHITE_QUEENSIDE: usize = 1;
pub const BLACK_KINGSIDE: usize = 2;
pub const BLACK_QUEENSIDE: usize = 3;

pub const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Board {
    pub squares: [Option<Piece>; 64],
    pub side_to_move: Color,
    /// castling still allowed: indexed by WHITE_KINGSIDE, WHITE_QUEENSIDE, ...
    pub castling: [bool; 4],
    /// the empty square a pawn just skipped with a double step (a capture target)
    pub en_passant: Option<u8>,
}

// Directions as (row change, column change). Row -1 is towards rank 8.
const KNIGHT_JUMPS: [(i8, i8); 8] =
    [(1, 2), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1), (-2, 1), (-1, 2)];
const ROOK_DIRS: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const BISHOP_DIRS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
const KING_DIRS: [(i8, i8); 8] =
    [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];
const PROMOTIONS: [Kind; 4] = [Kind::Queen, Kind::Rook, Kind::Bishop, Kind::Knight];

/// Square reached from `sq` by moving (d_row, d_col), or None if that is off the board.
pub fn offset(sq: u8, d_row: i8, d_col: i8) -> Option<u8> {
    let row = (sq / 8) as i8 + d_row;
    let col = (sq % 8) as i8 + d_col;
    if (0..8).contains(&row) && (0..8).contains(&col) {
        Some((row * 8 + col) as u8)
    } else {
        None
    }
}

fn kind_letter(kind: Kind) -> char {
    match kind {
        Kind::Pawn => 'p',
        Kind::Knight => 'n',
        Kind::Bishop => 'b',
        Kind::Rook => 'r',
        Kind::Queen => 'q',
        Kind::King => 'k',
    }
}

/// FEN letter to piece: uppercase is white, lowercase is black.
fn piece_from_letter(letter: char) -> Option<Piece> {
    let kind = match letter.to_ascii_lowercase() {
        'p' => Kind::Pawn,
        'n' => Kind::Knight,
        'b' => Kind::Bishop,
        'r' => Kind::Rook,
        'q' => Kind::Queen,
        'k' => Kind::King,
        _ => return None,
    };
    let color = if letter.is_ascii_uppercase() { Color::White } else { Color::Black };
    Some(Piece { color, kind })
}

/// Pawn moves onto the last rank become four moves, one per promotion piece.
fn push_pawn_move(from: u8, to: u8, promo_row: u8, moves: &mut Vec<Move>) {
    if to / 8 == promo_row {
        for kind in PROMOTIONS {
            moves.push(Move { from, to, promotion: Some(kind) });
        }
    } else {
        moves.push(Move { from, to, promotion: None });
    }
}

impl Board {
    pub fn start() -> Board {
        Board::from_fen(START_FEN).expect("start position FEN is valid")
    }

    /// Reads the first four fields of a FEN string (pieces, side to move, castling,
    /// en passant). Returns None if it can't be read.
    pub fn from_fen(fen: &str) -> Option<Board> {
        let mut fields = fen.split_whitespace();

        let mut squares = [None; 64];
        let mut sq = 0usize;
        for c in fields.next()?.chars() {
            match c {
                '/' => {}
                '1'..='8' => sq += c.to_digit(10)? as usize,
                _ => {
                    if sq >= 64 {
                        return None;
                    }
                    squares[sq] = Some(piece_from_letter(c)?);
                    sq += 1;
                }
            }
        }
        if sq != 64 {
            return None;
        }

        let side_to_move = match fields.next()? {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return None,
        };

        let mut castling = [false; 4];
        for c in fields.next().unwrap_or("-").chars() {
            match c {
                'K' => castling[WHITE_KINGSIDE] = true,
                'Q' => castling[WHITE_QUEENSIDE] = true,
                'k' => castling[BLACK_KINGSIDE] = true,
                'q' => castling[BLACK_QUEENSIDE] = true,
                _ => {}
            }
        }

        let en_passant = match fields.next().unwrap_or("-") {
            "-" => None,
            square => Some(chess_notation_utilities::notation_to_index(square).ok()? as u8),
        };

        Some(Board { squares, side_to_move, castling, en_passant })
    }

    /// Every legal move for the side to move.
    pub fn legal_moves(&self) -> Vec<Move> {
        let me = self.side_to_move;
        self.pseudo_legal_moves()
            .into_iter()
            // try the move; keep it only if our own king isn't attacked afterwards
            .filter(|&mv| !self.make_move(mv).in_check(me))
            .collect()
    }

    /// Plays `mv` and returns the new board. Does not check that the move is legal.
    pub fn make_move(&self, mv: Move) -> Board {
        let mut b = *self;
        let piece = b.squares[mv.from as usize]
            .take()
            .expect("make_move: no piece on the from square");

        // en passant: the captured pawn sits behind the destination square
        if piece.kind == Kind::Pawn && Some(mv.to) == self.en_passant {
            let captured = match piece.color {
                Color::White => mv.to + 8,
                Color::Black => mv.to - 8,
            };
            b.squares[captured as usize] = None;
        }

        // castling: the king moves two squares and the rook jumps over it
        if piece.kind == Kind::King && mv.from.abs_diff(mv.to) == 2 {
            let (rook_from, rook_to) = if mv.to > mv.from {
                (mv.from + 3, mv.from + 1)
            } else {
                (mv.from - 4, mv.from - 1)
            };
            b.squares[rook_to as usize] = b.squares[rook_from as usize].take();
        }

        b.squares[mv.to as usize] = Some(match mv.promotion {
            Some(kind) => Piece { color: piece.color, kind },
            None => piece,
        });

        // a double pawn step makes the skipped square an en passant target for one move
        b.en_passant = if piece.kind == Kind::Pawn && mv.from.abs_diff(mv.to) == 16 {
            Some((mv.from + mv.to) / 2)
        } else {
            None
        };

        // moving from or capturing on a rook's start square loses that castling right
        for (right, rook_square) in
            [(WHITE_KINGSIDE, 63u8), (WHITE_QUEENSIDE, 56), (BLACK_KINGSIDE, 7), (BLACK_QUEENSIDE, 0)]
        {
            if mv.from == rook_square || mv.to == rook_square {
                b.castling[right] = false;
            }
        }
        // moving the king loses both
        if piece.kind == Kind::King {
            let rights = match piece.color {
                Color::White => [WHITE_KINGSIDE, WHITE_QUEENSIDE],
                Color::Black => [BLACK_KINGSIDE, BLACK_QUEENSIDE],
            };
            for right in rights {
                b.castling[right] = false;
            }
        }

        b.side_to_move = b.side_to_move.opponent();
        b
    }

    /// True if `color`'s king is attacked.
    pub fn in_check(&self, color: Color) -> bool {
        let king = Some(Piece { color, kind: Kind::King });
        match self.squares.iter().position(|&p| p == king) {
            Some(sq) => self.is_attacked(sq as u8, color.opponent()),
            None => false,
        }
    }

    /// True if any piece of colour `by` attacks square `sq`.
    pub fn is_attacked(&self, sq: u8, by: Color) -> bool {
        let has = |square: Option<u8>, kind: Kind| {
            square.map_or(false, |s| self.squares[s as usize] == Some(Piece { color: by, kind }))
        };

        // pawns attack diagonally forwards, so look one row "behind" sq from by's side
        let pawn_row = match by {
            Color::White => 1,
            Color::Black => -1,
        };
        for d_col in [-1, 1] {
            if has(offset(sq, pawn_row, d_col), Kind::Pawn) {
                return true;
            }
        }
        for (d_row, d_col) in KNIGHT_JUMPS {
            if has(offset(sq, d_row, d_col), Kind::Knight) {
                return true;
            }
        }
        for (d_row, d_col) in KING_DIRS {
            if has(offset(sq, d_row, d_col), Kind::King) {
                return true;
            }
        }
        // sliders: only the first piece along each line can attack (the rest are blocked)
        for (dirs, kind) in [(&ROOK_DIRS, Kind::Rook), (&BISHOP_DIRS, Kind::Bishop)] {
            for &(d_row, d_col) in dirs {
                let mut s = sq;
                while let Some(next) = offset(s, d_row, d_col) {
                    if let Some(p) = self.squares[next as usize] {
                        if p.color == by && (p.kind == kind || p.kind == Kind::Queen) {
                            return true;
                        }
                        break;
                    }
                    s = next;
                }
            }
        }
        false
    }

    /// Moves that follow each piece's movement rules, ignoring whether they leave the
    /// king in check. `legal_moves` filters those out.
    fn pseudo_legal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::with_capacity(64);
        for from in 0..64u8 {
            let Some(piece) = self.squares[from as usize] else { continue };
            if piece.color != self.side_to_move {
                continue;
            }
            match piece.kind {
                Kind::Pawn => self.pawn_moves(from, &mut moves),
                Kind::Knight => self.step_moves(from, &KNIGHT_JUMPS, &mut moves),
                Kind::Bishop => self.slider_moves(from, &BISHOP_DIRS, &mut moves),
                Kind::Rook => self.slider_moves(from, &ROOK_DIRS, &mut moves),
                Kind::Queen => {
                    self.slider_moves(from, &ROOK_DIRS, &mut moves);
                    self.slider_moves(from, &BISHOP_DIRS, &mut moves);
                }
                Kind::King => {
                    self.step_moves(from, &KING_DIRS, &mut moves);
                    self.castling_moves(from, &mut moves);
                }
            }
        }
        moves
    }

    /// One jump or step in each direction (knights and kings), onto an empty or enemy square.
    fn step_moves(&self, from: u8, steps: &[(i8, i8)], moves: &mut Vec<Move>) {
        for &(d_row, d_col) in steps {
            if let Some(to) = offset(from, d_row, d_col) {
                if self.squares[to as usize].map_or(true, |p| p.color != self.side_to_move) {
                    moves.push(Move { from, to, promotion: None });
                }
            }
        }
    }

    /// Bishops, rooks and queens: walk each direction until the edge or a piece.
    fn slider_moves(&self, from: u8, dirs: &[(i8, i8)], moves: &mut Vec<Move>) {
        for &(d_row, d_col) in dirs {
            let mut sq = from;
            while let Some(to) = offset(sq, d_row, d_col) {
                match self.squares[to as usize] {
                    None => moves.push(Move { from, to, promotion: None }),
                    Some(p) => {
                        if p.color != self.side_to_move {
                            moves.push(Move { from, to, promotion: None }); // capture
                        }
                        break; // blocked either way
                    }
                }
                sq = to;
            }
        }
    }

    fn pawn_moves(&self, from: u8, moves: &mut Vec<Move>) {
        // white moves towards row 0 (rank 8), black towards row 7 (rank 1)
        let (dir, start_row, promo_row) = match self.side_to_move {
            Color::White => (-1i8, 6u8, 0u8),
            Color::Black => (1i8, 1u8, 7u8),
        };

        // forward: only onto empty squares (pawns can't capture forwards)
        if let Some(one) = offset(from, dir, 0) {
            if self.squares[one as usize].is_none() {
                push_pawn_move(from, one, promo_row, moves);
                // double step from the start row; `one` is already known to be empty
                if from / 8 == start_row {
                    if let Some(two) = offset(one, dir, 0) {
                        if self.squares[two as usize].is_none() {
                            moves.push(Move { from, to: two, promotion: None });
                        }
                    }
                }
            }
        }

        // diagonal captures, including en passant onto the empty skipped square
        for d_col in [-1i8, 1] {
            if let Some(to) = offset(from, dir, d_col) {
                let enemy_there =
                    self.squares[to as usize].map_or(false, |p| p.color != self.side_to_move);
                if enemy_there || Some(to) == self.en_passant {
                    push_pawn_move(from, to, promo_row, moves);
                }
            }
        }
    }

    /// Castling: the right must still be held, the squares between king and rook must be
    /// empty, and the king can't start in or cross an attacked square. The landing square
    /// is checked by the filter in `legal_moves`, like any other king move.
    fn castling_moves(&self, from: u8, moves: &mut Vec<Move>) {
        let (home, kingside, queenside) = match self.side_to_move {
            Color::White => (60u8, WHITE_KINGSIDE, WHITE_QUEENSIDE),
            Color::Black => (4u8, BLACK_KINGSIDE, BLACK_QUEENSIDE),
        };
        if from != home {
            return;
        }
        let enemy = self.side_to_move.opponent();
        let empty = |sq: u8| self.squares[sq as usize].is_none();

        if self.castling[kingside]
            && empty(home + 1)
            && empty(home + 2)
            && !self.is_attacked(home, enemy)
            && !self.is_attacked(home + 1, enemy)
        {
            moves.push(Move { from, to: home + 2, promotion: None });
        }
        if self.castling[queenside]
            && empty(home - 1)
            && empty(home - 2)
            && empty(home - 3)
            && !self.is_attacked(home, enemy)
            && !self.is_attacked(home - 1, enemy)
        {
            moves.push(Move { from, to: home - 2, promotion: None });
        }
    }
}

/// Builds an AI board from the main game's state.
impl From<&GameState> for Board {
    fn from(state: &GameState) -> Board {
        let mut squares = [None; 64];
        for (i, square) in squares.iter_mut().enumerate() {
            if let Some(p) = state.get_piece_at(i) {
                *square = piece_from_letter(visual::get_piece_char_from_unicode(p.get_unicode_val()));
            }
        }

        // a castling right needs the king and that rook on their start squares, unmoved
        let unmoved = |sq: usize, wanted: Piece| {
            squares[sq] == Some(wanted) && state.get_piece_at(sq).map_or(false, |p| !p.get_moved())
        };
        let mut castling = [false; 4];
        for (right, color, king_sq, rook_sq) in [
            (WHITE_KINGSIDE, Color::White, 60, 63),
            (WHITE_QUEENSIDE, Color::White, 60, 56),
            (BLACK_KINGSIDE, Color::Black, 4, 7),
            (BLACK_QUEENSIDE, Color::Black, 4, 0),
        ] {
            castling[right] = unmoved(king_sq, Piece { color, kind: Kind::King })
                && unmoved(rook_sq, Piece { color, kind: Kind::Rook });
        }

        // GameState stores en passant captures as "<from>-<to><captured>"; the target is <to>
        let en_passant = state
            .en_passant_enabled
            .as_ref()
            .and_then(|moves| moves.first())
            .and_then(|m| m.get(3..5))
            .and_then(|to| chess_notation_utilities::notation_to_index(to).ok())
            .map(|sq| sq as u8);

        let side_to_move = match state.player_turn {
            PLAYER::WHITE => Color::White,
            PLAYER::BLACK => Color::Black,
        };

        Board { squares, side_to_move, castling, en_passant }
    }
}

/// Prints the board with FEN letters, rank 8 at the top.
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..8 {
            write!(f, "{} ", 8 - row)?;
            for col in 0..8 {
                let c = match self.squares[row * 8 + col] {
                    Some(p) if p.color == Color::White => kind_letter(p.kind).to_ascii_uppercase(),
                    Some(p) => kind_letter(p.kind),
                    None => '.',
                };
                write!(f, "{} ", c)?;
            }
            writeln!(f)?;
        }
        write!(f, "  a b c d e f g h")
    }
}

/// Counts every sequence of legal moves `depth` moves deep. Comparing the result with
/// known values is the standard test that move generation is correct.
pub fn perft(board: &Board, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }
    board
        .legal_moves()
        .iter()
        .map(|&mv| perft(&board.make_move(mv), depth - 1))
        .sum()
}

/// `perft` split by first move, for tracking down a wrong count: compare each line with
/// a trusted engine (e.g. Stockfish's `go perft <depth>`) and dig into the one that differs.
pub fn perft_divide(board: &Board, depth: u32) -> Vec<(String, u64)> {
    board
        .legal_moves()
        .iter()
        .map(|&mv| (mv.to_notation(), perft(&board.make_move(mv), depth.saturating_sub(1))))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Game;

    fn perft_fen(fen: &str, depth: u32) -> u64 {
        perft(&Board::from_fen(fen).expect("valid FEN"), depth)
    }

    #[test]
    fn perft_start_position() {
        let b = Board::start();
        assert_eq!(perft(&b, 1), 20);
        assert_eq!(perft(&b, 2), 400);
        assert_eq!(perft(&b, 3), 8_902);
        assert_eq!(perft(&b, 4), 197_281);
    }

    /// "Kiwipete": lots of castling, en passant, pins and promotions-to-be.
    #[test]
    fn perft_kiwipete() {
        let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
        assert_eq!(perft_fen(fen, 1), 48);
        assert_eq!(perft_fen(fen, 2), 2_039);
        assert_eq!(perft_fen(fen, 3), 97_862);
    }

    /// Endgame with en passant that would expose the king along a rank.
    #[test]
    fn perft_position_3() {
        let fen = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
        assert_eq!(perft_fen(fen, 1), 14);
        assert_eq!(perft_fen(fen, 2), 191);
        assert_eq!(perft_fen(fen, 3), 2_812);
        assert_eq!(perft_fen(fen, 4), 43_238);
    }

    /// Promotions and castling rights.
    #[test]
    fn perft_position_4() {
        let fen = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
        assert_eq!(perft_fen(fen, 1), 6);
        assert_eq!(perft_fen(fen, 2), 264);
        assert_eq!(perft_fen(fen, 3), 9_467);
    }

    #[test]
    fn converts_game_state() {
        assert_eq!(Board::from(&Game::default().state), Board::start());

        // after 1.e4 the side to move flips; the main game's moves should be legal here too
        let game = Game::game_from_turn_history(&["e2-e4", "c7-c5", "e4-e5", "d7-d5"]);
        let board = Board::from(&game.state);
        assert_eq!(board.side_to_move, Color::White);
        // en passant e5xd6 is available
        assert!(board.legal_moves().iter().any(|m| m.to_notation() == "e5-d6"));
    }

    #[test]
    fn move_notation() {
        let mv = Move { from: 52, to: 36, promotion: None };
        assert_eq!(mv.to_notation(), "e2-e4");
        let promo = Move { from: 12, to: 4, promotion: Some(Kind::Knight) };
        assert_eq!(promo.to_notation(), "e7-e8=N");
    }
}
