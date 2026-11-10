//! Position evaluation: material plus piece-square tables.
//!
//! Scores are in centipawns (100 = one pawn) from white's point of view:
//! positive is good for white, negative is good for black.
//!
//! A piece-square table gives each piece a small bonus or penalty depending on its
//! square, e.g. knights are worth more in the centre than in a corner. The values are
//! from Tomasz Michniewski's "Simplified Evaluation Function", a standard starting point.
//!
//! Tables are written from white's side with rank 8 on the top row, which matches the
//! board's square numbers (0 = a8 ... 63 = h1), so white reads `table[sq]` directly.
//! Black reads the vertically mirrored square `sq ^ 56` (a8 <-> a1, h7 <-> h2, ...).

use super::board::{Board, Color, Kind};

pub fn piece_value(kind: Kind) -> i32 {
    match kind {
        Kind::Pawn => 100,
        Kind::Knight => 320,
        Kind::Bishop => 330,
        Kind::Rook => 500,
        Kind::Queen => 900,
        Kind::King => 0, // never captured, so it doesn't count
    }
}

#[rustfmt::skip]
const PAWN_TABLE: [i32; 64] = [
     0,  0,  0,  0,  0,  0,  0,  0,   // rank 8 (a pawn never stands here)
    50, 50, 50, 50, 50, 50, 50, 50,   // rank 7: about to promote
    10, 10, 20, 30, 30, 20, 10, 10,
     5,  5, 10, 25, 25, 10,  5,  5,
     0,  0,  0, 20, 20,  0,  0,  0,   // rank 4: central pawns rewarded
     5, -5,-10,  0,  0,-10, -5,  5,
     5, 10, 10,-20,-20, 10, 10,  5,   // rank 2: d/e pawns should move, f/g/h shield the king
     0,  0,  0,  0,  0,  0,  0,  0,   // rank 1
];

#[rustfmt::skip]
const KNIGHT_TABLE: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,  // "a knight on the rim is dim"
];

#[rustfmt::skip]
const BISHOP_TABLE: [i32; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

#[rustfmt::skip]
const ROOK_TABLE: [i32; 64] = [
     0,  0,  0,  0,  0,  0,  0,  0,
     5, 10, 10, 10, 10, 10, 10,  5,   // rook on the 7th rank
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
     0,  0,  0,  5,  5,  0,  0,  0,   // central files after castling
];

#[rustfmt::skip]
const QUEEN_TABLE: [i32; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
     -5,  0,  5,  5,  5,  5,  0, -5,
      0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20,
];

/// King while there are pieces on the board: stay castled behind the pawns.
#[rustfmt::skip]
const KING_MIDDLEGAME_TABLE: [i32; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
     20, 20,  0,  0,  0,  0, 20, 20,
     20, 30, 10,  0,  0, 10, 30, 20,  // g1/b1 (after castling) best
];

/// King in the endgame: it becomes a fighting piece and belongs in the centre.
#[rustfmt::skip]
const KING_ENDGAME_TABLE: [i32; 64] = [
    -50,-40,-30,-20,-20,-30,-40,-50,
    -30,-20,-10,  0,  0,-10,-20,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-30,  0,  0,  0,  0,-30,-30,
    -50,-30,-30,-30,-30,-30,-30,-50,
];

/// Non-pawn material (both sides) at the start: 2 x (2 knights + 2 bishops + 2 rooks + queen).
const OPENING_PHASE: i32 = 2 * (2 * 320 + 2 * 330 + 2 * 500 + 900);

/// The square to look up in a white-side table for a piece of `color` on `sq`.
fn table_index(sq: usize, color: Color) -> usize {
    match color {
        Color::White => sq,
        Color::Black => sq ^ 56, // flip the row, keep the column
    }
}

/// How far the game is from the endgame: OPENING_PHASE with all pieces on the board,
/// falling towards 0 as knights, bishops, rooks and queens are traded (pawns don't count).
fn game_phase(board: &Board) -> i32 {
    let material: i32 = board
        .squares
        .iter()
        .flatten()
        .filter(|p| p.kind != Kind::Pawn && p.kind != Kind::King)
        .map(|p| piece_value(p.kind))
        .sum();
    material.min(OPENING_PHASE) // promotions can push it above the start value
}

/// Material only: white's pieces minus black's.
pub fn material(board: &Board) -> i32 {
    board
        .squares
        .iter()
        .flatten()
        .map(|p| {
            let value = piece_value(p.kind);
            if p.color == Color::White { value } else { -value }
        })
        .sum()
}

/// Material plus piece-square tables. The king blends its middlegame and endgame
/// tables according to how much material is left (a "tapered" evaluation).
pub fn evaluate(board: &Board) -> i32 {
    let phase = game_phase(board);
    let mut score = 0;
    for (sq, square) in board.squares.iter().enumerate() {
        let Some(piece) = square else { continue };
        let i = table_index(sq, piece.color);
        let positional = match piece.kind {
            Kind::Pawn => PAWN_TABLE[i],
            Kind::Knight => KNIGHT_TABLE[i],
            Kind::Bishop => BISHOP_TABLE[i],
            Kind::Rook => ROOK_TABLE[i],
            Kind::Queen => QUEEN_TABLE[i],
            Kind::King => {
                (KING_MIDDLEGAME_TABLE[i] * phase + KING_ENDGAME_TABLE[i] * (OPENING_PHASE - phase))
                    / OPENING_PHASE
            }
        };
        let value = piece_value(piece.kind) + positional;
        score += if piece.color == Color::White { value } else { -value };
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::board::Piece;

    fn board(fen: &str) -> Board {
        Board::from_fen(fen).expect("valid FEN")
    }

    /// The same position with colours swapped and the board turned upside down.
    fn mirrored(b: &Board) -> Board {
        let mut m = *b;
        for sq in 0..64 {
            m.squares[sq ^ 56] = b.squares[sq].map(|p| Piece { color: p.color.opponent(), kind: p.kind });
        }
        m.side_to_move = b.side_to_move.opponent();
        m
    }

    #[test]
    fn start_position_is_even() {
        assert_eq!(material(&Board::start()), 0);
        assert_eq!(evaluate(&Board::start()), 0);
    }

    /// Black's tables must be exact mirrors of white's: swapping colours flips the sign.
    #[test]
    fn evaluation_is_symmetric() {
        for fen in [
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
            "rnbqkb1r/pppp1ppp/5n2/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 2 3",
        ] {
            let b = board(fen);
            assert_eq!(evaluate(&b), -evaluate(&mirrored(&b)), "fen {}", fen);
        }
    }

    #[test]
    fn central_knight_beats_corner_knight() {
        let centre = board("4k3/8/8/8/4N3/8/8/4K3 w - - 0 1");
        let corner = board("4k3/8/8/8/8/8/8/N3K3 w - - 0 1");
        assert!(evaluate(&centre) > evaluate(&corner));
    }

    #[test]
    fn developing_moves_score_better_than_rim_moves() {
        let developed = Board::start().make_move(move_from("g1", "f3"));
        let rim = Board::start().make_move(move_from("g1", "h3"));
        assert!(evaluate(&developed) > evaluate(&rim));
    }

    #[test]
    fn king_prefers_centre_in_endgame_and_safety_in_middlegame() {
        // endgame (kings and pawns only): a central king is better
        let central = board("4k3/pppp4/8/8/4K3/8/PPPP4/8 w - - 0 1");
        let corner = board("4k3/pppp4/8/8/8/8/PPPP4/7K w - - 0 1");
        assert!(evaluate(&central) > evaluate(&corner));
        // middlegame (all pieces): a castled king is better than one in the centre
        let castled = board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQ1RK1 w kq - 0 1");
        let exposed = board("rnbqkbnr/pppppppp/8/8/4K3/8/PPPPPPPP/RNBQ1R2 w kq - 0 1");
        assert!(evaluate(&castled) > evaluate(&exposed));
    }

    fn move_from(from: &str, to: &str) -> crate::ai::board::Move {
        use crate::chess_notation_utilities::notation_to_index;
        crate::ai::board::Move {
            from: notation_to_index(from).unwrap() as u8,
            to: notation_to_index(to).unwrap() as u8,
            promotion: None,
        }
    }
}
