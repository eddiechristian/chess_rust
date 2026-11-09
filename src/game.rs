use crate::chess_errors;
use crate::chess_notation_utilities;
use crate::visual;

use std::io::{stdin, stdout, Write};
use std::{env, io};
use visual::{GameState, PLAYER};

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

/// Result of looking at the position from the point of view of the player to move.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameStatus {
    InProgress,
    /// the player to move is in check but has a legal move
    Check,
    /// the player to move is in check with no legal moves; holds the winner
    Checkmate(PLAYER),
    /// the player to move is not in check but has no legal moves
    Stalemate,
    /// 50 moves by each player without a pawn move or a capture
    FiftyMoveDraw,
}

/// Halfmoves (one player's move) without a pawn move or capture that end the game in a draw.
pub const FIFTY_MOVE_RULE_HALFMOVES: u32 = 100;

pub struct Game {
    pub state: GameState,
    pub turn_history: Vec<String>,
    /// halfmoves since the last pawn move or capture, for the 50-move rule
    pub halfmove_clock: u32,
}
impl Default for Game {
    fn default() -> Self {
        Game {
            state: GameState::default(),
            turn_history: Vec::new(),
            halfmove_clock: 0,
        }
    }
}
impl Game {
    pub fn game_from_turn_history(turn_history: &[&str]) -> Self {
        let mut chess_game = Game {
            state: GameState::default(),
            turn_history: Vec::new(),
            halfmove_clock: 0,
        };
        for turn in turn_history {
            println!("turn {}", turn);
            chess_game.turn_history.push(turn.to_string());
            if let Err(e) = chess_game.move_piece(&turn, chess_game.state.player_turn) {
                println!("{}", e);
            } else {
                chess_game.state.player_turn = match chess_game.state.player_turn {
                    visual::PLAYER::WHITE => visual::PLAYER::BLACK,
                    visual::PLAYER::BLACK => visual::PLAYER::WHITE,
                };
            }
        }
        chess_game
    }
    pub fn check_pieces_between(
        &self,
        from_spot: &str,
        to_spot: &str,
        dir: Direction,
    ) -> Result<(), chess_errors::ChessErrors> {
        let mut pos: String = to_spot.to_string();
        loop {
            if let Ok(bounds) = chess_notation_utilities::get_bounds(&pos) {
                let next_pos_opt = match dir {
                    Direction::Up => bounds.bottom,
                    Direction::Down => bounds.top,
                    Direction::Left => bounds.right,
                    Direction::Right => bounds.left,
                    Direction::DownLeft => bounds.top_right_diag,
                    Direction::UpLeft => bounds.bottom_right_diag,
                    Direction::UpRight => bounds.bottom_left_diag,
                    Direction::DownRight => bounds.top_left_diag,
                };
                if let Some(next_pos_array) = next_pos_opt {
                    let x = std::str::from_utf8(&next_pos_array).unwrap();
                    pos = x.to_string();
                    if pos == from_spot {
                        break;
                    }
                    if let Ok(index) = chess_notation_utilities::notation_to_index(&pos) {
                        // any piece in between blocks the move, whoever owns it
                        if self.state.get_piece_at(index).is_some() {
                            return Err(chess_errors::ChessErrors::PieceBetween(pos));
                        }
                    }
                } else {
                    println!("oops1");
                    return Err(chess_errors::ChessErrors::InvalidNotation(pos.to_string()));
                }
            } else {
                println!("oops2");
                return Err(chess_errors::ChessErrors::InvalidNotation(pos.to_string()));
            }
        }
        Ok(())
    }
    pub fn get_validated_moves(&self, player: PLAYER) -> Vec<String> {
        let mut validated_moves = Vec::new();
        let mut unvalidated_moves = self.state.get_unvalidated_moves(player);
        for (index, piece_move) in unvalidated_moves.iter().enumerate() {
            let move_spots: Vec<&str> = piece_move.split("-").collect();
            if self
                .is_move_valid(move_spots[0], move_spots[1], player, None)
                .is_ok()
            {
                validated_moves.push(piece_move.clone());
            }
        }
        println!("validated_moves: {:?}", validated_moves);
        validated_moves
    }

    /// True if `player`'s king is currently attacked.
    pub fn is_in_check(&self, player: PLAYER) -> bool {
        self.check_king_safe(&self.glyph_board(), player).is_err()
    }

    /// Check, checkmate or stalemate for the player whose turn it is.
    pub fn game_status(&self) -> GameStatus {
        let player = self.state.player_turn;
        let in_check = self.is_in_check(player);
        let has_legal_move = self.state.get_unvalidated_moves(player).iter().any(|piece_move| {
            let move_spots: Vec<&str> = piece_move.split("-").collect();
            self.is_move_valid(move_spots[0], move_spots[1], player, None)
                .is_ok()
        });
        match (in_check, has_legal_move) {
            // checkmate and stalemate take priority over the 50-move rule
            (true, false) => GameStatus::Checkmate(match player {
                PLAYER::WHITE => PLAYER::BLACK,
                PLAYER::BLACK => PLAYER::WHITE,
            }),
            (false, false) => GameStatus::Stalemate,
            _ if self.halfmove_clock >= FIFTY_MOVE_RULE_HALFMOVES => GameStatus::FiftyMoveDraw,
            (true, true) => GameStatus::Check,
            (false, true) => GameStatus::InProgress,
        }
    }

    pub fn is_move_valid(
        &self,
        from_spot: &str,
        to_spot: &str,
        whos_turn: visual::PLAYER,
        promotion_opt: Option<&str>,
    ) -> Result<(visual::MoveType), chess_errors::ChessErrors> {
        // first determine if piece at from is correct player.
        if let Ok(index) = chess_notation_utilities::notation_to_index(&from_spot) {
            if let Some(piece) = self.state.get_piece_at(index) {
                if piece.get_player() != whos_turn {
                    let msg = format!("{}", from_spot);
                    return Err(chess_errors::ChessErrors::WrongPlayer(msg));
                }
            } else {
                let msg = format!("{}", from_spot);
                return Err(chess_errors::ChessErrors::NoPiece(msg));
            }
        }
        //if too spot is current player its invalid
        if let Ok(index) = chess_notation_utilities::notation_to_index(&to_spot) {
            if let Some(piece) = self.state.get_piece_at(index) {
                if piece.get_player() == whos_turn {
                    let msg = format!("{}", to_spot);
                    return Err(chess_errors::ChessErrors::PlayerPieceAlreadyThere(msg));
                }
            }
        }
        if promotion_opt.is_some() {
            //promotions are only valid from 8th rank for pawn
            let from_row = chess_notation_utilities::convert_row(from_spot)?;
            let to_row = chess_notation_utilities::convert_row(to_spot)?;
            // a promoting pawn moves from the 7th to the 8th rank (2nd to 1st for black)
            let (promo_from_row, promo_to_row) = match whos_turn {
                PLAYER::WHITE => (1, 0),
                PLAYER::BLACK => (6, 7),
            };
            if to_row != promo_to_row || from_row != promo_from_row {
                let msg = format!("{}", to_spot);
                return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
            }
        }
        //check move against en-passant_moves
        if let Some(en_passant_enabled_vec) = &self.state.en_passant_enabled {
            println!("en_passant_enabled_vec: {:?}", en_passant_enabled_vec);
            for en_passant_move in en_passant_enabled_vec {
                let en_passant_notation_move = &en_passant_move[0..5];
                println!("en_passant_notation_move: {:?}", en_passant_notation_move);
                let notation_move = format!("{}-{}", from_spot, to_spot);
                if en_passant_notation_move == notation_move {
                    let attacked_piece = &en_passant_move[5..];
                    if let Ok(index) = chess_notation_utilities::notation_to_index(&attacked_piece)
                    {
                        self.check_move_safety(from_spot, to_spot, whos_turn, Some(index))?;
                        return Ok(visual::MoveType::Enpassant(index));
                    }
                }
            }
        }

        // castling: the king moves two squares along its own rank
        {
            let from = chess_notation_utilities::notation_to_index(from_spot)?;
            let to = chess_notation_utilities::notation_to_index(to_spot)?;
            let own_king = match whos_turn {
                PLAYER::WHITE => visual::WHITE_KING,
                PLAYER::BLACK => visual::BLACK_KING,
            };
            let is_king = self
                .state
                .get_piece_at(from)
                .map_or(false, |p| p.get_unicode_val() == own_king);
            if is_king && from / 8 == to / 8 && (from as isize - to as isize).abs() == 2 {
                if promotion_opt.is_some() {
                    return Err(chess_errors::ChessErrors::InvalidPromotion(to_spot.to_string()));
                }
                return self.check_castling(from, to, whos_turn);
            }
        }

        // the x and y deltas will tell what kind of move it is

        let (from_point, to_point) =
            chess_notation_utilities::convert_move_notation_to_xy(from_spot, to_spot)?;
        let delta_x: i8 = (from_point.x as i8 - to_point.x as i8) as i8;
        let delta_y: i8 = (from_point.y as i8 - to_point.y as i8) as i8;
        if delta_x == 0 {
            //vertical
            let dir = {
                if delta_y < 0 {
                    Direction::Down
                } else {
                    Direction::Up
                }
            };
            if delta_y.abs() != 1 {
                self.check_pieces_between(from_spot, to_spot, dir)?;
            }
            if let Ok(index) = chess_notation_utilities::notation_to_index(&from_spot) {
                if let Some(piece) = self.state.get_piece_at(index) {
                    if let (_, visual::MoveType::Promotion(new_piece)) =
                        piece.move_vertical(to_spot, &self.state, delta_y, promotion_opt)?
                    {
                        self.check_move_safety(from_spot, to_spot, whos_turn, None)?;
                        return Ok(visual::MoveType::Promotion(new_piece));
                    }
                }
            }
        } else if delta_y == 0 {
            //Horiz
            let dir = {
                if delta_x < 0 {
                    Direction::Right
                } else {
                    Direction::Left
                }
            };
            if delta_x.abs() != 1 {
                self.check_pieces_between(from_spot, to_spot, dir)?;
            }
            if let Ok(index) = chess_notation_utilities::notation_to_index(&from_spot) {
                if let Some(piece) = self.state.get_piece_at(index) {
                    piece.move_horizontal(to_spot, &self.state, delta_x, promotion_opt)?;
                }
            }
        } else if delta_x.abs() == delta_y.abs() {
            //diagonal
            //determine dir
            let dir = {
                if delta_x > 0 && delta_y > 0 {
                    Direction::UpLeft
                } else if delta_x > 0 && delta_y < 0 {
                    Direction::DownLeft
                } else if delta_x < 0 && delta_y < 0 {
                    Direction::DownRight
                } else {
                    Direction::UpRight
                }
            };
            if delta_x.abs() != 1 {
                //check pieces between because multiple spaces
                self.check_pieces_between(from_spot, to_spot, dir)?;
            }
            if let Ok(index) = chess_notation_utilities::notation_to_index(&from_spot) {
                if let Some(piece) = self.state.get_piece_at(index) {
                    piece.move_diagonal(to_spot, &self.state, delta_y, promotion_opt)?;
                    // a pawn capturing onto the last rank promotes
                    let pawn = match whos_turn {
                        PLAYER::WHITE => visual::WHITE_PAWN,
                        PLAYER::BLACK => visual::BLACK_PAWN,
                    };
                    let to_row = chess_notation_utilities::convert_row(to_spot)?;
                    if piece.get_unicode_val() == pawn && to_row == visual::promotion_row(whos_turn) {
                        self.check_move_safety(from_spot, to_spot, whos_turn, None)?;
                        return Ok(visual::MoveType::Promotion(visual::promotion_glyph(
                            whos_turn,
                            promotion_opt,
                        )?));
                    }
                }
            }
            // if diagonal deltas must be equal, except for Knight
        } else if (delta_x.abs() == 2 && delta_y.abs() == 1)
            || (delta_x.abs() == 1 && delta_y.abs() == 2)
        {
            if let Ok(index) = chess_notation_utilities::notation_to_index(&from_spot) {
                if let Some(piece) = self.state.get_piece_at(index) {
                    piece.move_knight(to_spot, &self.state, promotion_opt)?;
                }
            }
        }

        //check for pinned piece, then that the move doesn't leave the king in check
        self.check_move_safety(from_spot, to_spot, whos_turn, None)?;

        Ok(visual::MoveType::Regular)
    }

    /// Runs the pin check, then plays the move on a copy of the board and returns
    /// `KingInCheck` if `whos_turn`'s king would be attacked afterwards.
    /// `captured` is the square of a piece removed by en passant, if any.
    pub fn check_move_safety(
        &self,
        from_spot: &str,
        to_spot: &str,
        whos_turn: visual::PLAYER,
        captured: Option<usize>,
    ) -> Result<(), chess_errors::ChessErrors> {
        self.check_pinned(from_spot, to_spot, whos_turn)?;
        let from = chess_notation_utilities::notation_to_index(from_spot)?;
        let to = chess_notation_utilities::notation_to_index(to_spot)?;
        let mut board = self.glyph_board();
        board[to] = board[from].take();
        if let Some(index) = captured {
            board[index] = None;
        }
        self.check_king_safe(&board, whos_turn)
    }

    /// Returns `KingInCheck` if `whos_turn`'s king is attacked on `board`.
    fn check_king_safe(
        &self,
        board: &[Option<char>; 64],
        whos_turn: visual::PLAYER,
    ) -> Result<(), chess_errors::ChessErrors> {
        let own_king = match whos_turn {
            PLAYER::WHITE => visual::WHITE_KING,
            PLAYER::BLACK => visual::BLACK_KING,
        };
        if let Some(king) = board.iter().position(|&g| g == Some(own_king)) {
            if let Some(attacker) = Self::find_attacker(board, king, whos_turn) {
                return Err(chess_errors::ChessErrors::KingInCheck(
                    chess_notation_utilities::index_to_spot(attacker),
                ));
            }
        }
        Ok(())
    }

    /// The board as piece glyphs, for trying out moves without touching the real state.
    fn glyph_board(&self) -> [Option<char>; 64] {
        let mut board = [None; 64];
        for (index, square) in board.iter_mut().enumerate() {
            *square = self.state.get_piece_at(index).map(|p| p.get_unicode_val());
        }
        board
    }

    /// Returns the index of an enemy piece (enemy of `defender`) attacking `target` on `board`.
    fn find_attacker(board: &[Option<char>; 64], target: usize, defender: visual::PLAYER) -> Option<usize> {
        let (pawn, knight, bishop, rook, queen, king, pawn_row_offset) = match defender {
            // white pawns sit on higher row indexes and attack towards row 0
            PLAYER::BLACK => (
                visual::WHITE_PAWN,
                visual::WHITE_KNIGHT,
                visual::WHITE_BISHOP,
                visual::WHITE_ROOK,
                visual::WHITE_QUEEN,
                visual::WHITE_KING,
                1i8,
            ),
            PLAYER::WHITE => (
                visual::BLACK_PAWN,
                visual::BLACK_KNIGHT,
                visual::BLACK_BISHOP,
                visual::BLACK_ROOK,
                visual::BLACK_QUEEN,
                visual::BLACK_KING,
                -1i8,
            ),
        };
        let (t_row, t_col) = ((target / 8) as i8, (target % 8) as i8);
        let on_board = |r: i8, c: i8| (0..8).contains(&r) && (0..8).contains(&c);
        let index_of = |r: i8, c: i8| (r * 8 + c) as usize;
        let piece_at = |r: i8, c: i8| board[index_of(r, c)];
        let matches = |r: i8, c: i8, glyph: char| on_board(r, c) && piece_at(r, c) == Some(glyph);

        // pawns
        for d_col in [-1i8, 1] {
            let (r, c) = (t_row + pawn_row_offset, t_col + d_col);
            if matches(r, c, pawn) {
                return Some(index_of(r, c));
            }
        }
        // knights
        for (d_row, d_col) in [(1i8, 2i8), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1), (-2, 1), (-1, 2)] {
            let (r, c) = (t_row + d_row, t_col + d_col);
            if matches(r, c, knight) {
                return Some(index_of(r, c));
            }
        }
        // enemy king on an adjacent square
        for d_row in -1i8..=1 {
            for d_col in -1i8..=1 {
                let (r, c) = (t_row + d_row, t_col + d_col);
                if (d_row, d_col) != (0, 0) && matches(r, c, king) {
                    return Some(index_of(r, c));
                }
            }
        }
        // sliding pieces: first piece along each line
        for (d_row, d_col) in [(1i8, 0i8), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)] {
            let straight = d_row == 0 || d_col == 0;
            let (mut r, mut c) = (t_row + d_row, t_col + d_col);
            while on_board(r, c) {
                if let Some(glyph) = piece_at(r, c) {
                    let attacks = glyph == queen
                        || (straight && glyph == rook)
                        || (!straight && glyph == bishop);
                    if attacks {
                        return Some(index_of(r, c));
                    }
                    break;
                }
                r += d_row;
                c += d_col;
            }
        }
        None
    }

    /// Validates castling: the king moves two squares towards an unmoved rook on its own rank.
    /// The king and rook must not have moved, the squares between them must be empty, and the
    /// king may not be in check, pass through an attacked square, or land on one.
    fn check_castling(
        &self,
        from: usize,
        to: usize,
        whos_turn: visual::PLAYER,
    ) -> Result<visual::MoveType, chess_errors::ChessErrors> {
        let to_spot = chess_notation_utilities::index_to_spot(to);
        let invalid = || chess_errors::ChessErrors::InvalidMove(to_spot.clone());
        let own_rook = match whos_turn {
            PLAYER::WHITE => visual::WHITE_ROOK,
            PLAYER::BLACK => visual::BLACK_ROOK,
        };
        let king = self.state.get_piece_at(from).ok_or_else(invalid)?;
        if king.get_moved() {
            return Err(invalid());
        }
        let row_start = from - from % 8;
        let rook_index = if to > from { row_start + 7 } else { row_start };
        match self.state.get_piece_at(rook_index) {
            Some(rook) if rook.get_unicode_val() == own_rook && !rook.get_moved() => {}
            _ => return Err(invalid()),
        }
        let (low, high) = if rook_index > from { (from, rook_index) } else { (rook_index, from) };
        for index in low + 1..high {
            if self.state.get_piece_at(index).is_some() {
                return Err(chess_errors::ChessErrors::PieceBetween(
                    chess_notation_utilities::index_to_spot(index),
                ));
            }
        }
        // the king's start, the square it crosses and where it lands must all be safe
        let step: isize = if to > from { 1 } else { -1 };
        let mut board = self.glyph_board();
        let king_glyph = board[from].take();
        for offset in 0..=2isize {
            let square = (from as isize + step * offset) as usize;
            board[square] = king_glyph;
            self.check_king_safe(&board, whos_turn)?;
            board[square] = None;
        }
        Ok(visual::MoveType::Castling)
    }

    /// Returns `PieceIsPinned` if moving the piece on `from_spot` to `to_spot` would
    /// expose `whos_turn`'s king to an enemy rook, bishop or queen along a line.
    /// Moves that stay on the pin line (towards the king or capturing the pinner) are allowed.
    pub fn check_pinned(
        &self,
        from_spot: &str,
        to_spot: &str,
        whos_turn: visual::PLAYER,
    ) -> Result<(), chess_errors::ChessErrors> {
        let from = chess_notation_utilities::notation_to_index(from_spot)?;
        let to = chess_notation_utilities::notation_to_index(to_spot)?;
        let (own_king, enemy_rook, enemy_bishop, enemy_queen) = match whos_turn {
            PLAYER::WHITE => (
                visual::WHITE_KING,
                visual::BLACK_ROOK,
                visual::BLACK_BISHOP,
                visual::BLACK_QUEEN,
            ),
            PLAYER::BLACK => (
                visual::BLACK_KING,
                visual::WHITE_ROOK,
                visual::WHITE_BISHOP,
                visual::WHITE_QUEEN,
            ),
        };
        let king = match (0..64).find(|&i| {
            self.state
                .get_piece_at(i)
                .map_or(false, |p| p.get_unicode_val() == own_king)
        }) {
            Some(king) => king,
            None => return Ok(()),
        };
        // the king itself can't be pinned
        if king == from {
            return Ok(());
        }

        // the moving piece must share a row, column or diagonal with the king
        let (king_row, king_col) = ((king / 8) as i8, (king % 8) as i8);
        let (from_row, from_col) = ((from / 8) as i8, (from % 8) as i8);
        let (d_row, d_col) = (from_row - king_row, from_col - king_col);
        let straight = d_row == 0 || d_col == 0;
        if !straight && d_row.abs() != d_col.abs() {
            return Ok(());
        }

        // walk from the king through from_spot and look for an enemy slider
        let (step_row, step_col) = (d_row.signum(), d_col.signum());
        let (mut row, mut col) = (king_row + step_row, king_col + step_col);
        let mut passed_from = false;
        while (0..8).contains(&row) && (0..8).contains(&col) {
            let index = (row * 8 + col) as usize;
            if index == to {
                // the piece stays on the line, so the king stays shielded
                return Ok(());
            }
            if index == from {
                passed_from = true;
            } else if let Some(piece) = self.state.get_piece_at(index) {
                if !passed_from {
                    // another piece already shields the king
                    return Ok(());
                }
                let unicode = piece.get_unicode_val();
                let is_pinner = unicode == enemy_queen
                    || (straight && unicode == enemy_rook)
                    || (!straight && unicode == enemy_bishop);
                if is_pinner {
                    return Err(chess_errors::ChessErrors::PieceIsPinned(
                        chess_notation_utilities::index_to_spot(index),
                    ));
                }
                return Ok(());
            }
            row += step_row;
            col += step_col;
        }
        Ok(())
    }

    pub fn check_en_passant(
        &mut self,
        from_spot: &str,
        to_spot: &str,
        whos_turn: visual::PLAYER,
    ) -> Result<(), chess_errors::ChessErrors> {
        //in this function you see if last move is cause for enpassant then add moves to vec
        let mut vec_en_passant_moves = Vec::new();
        if let Ok(index) = chess_notation_utilities::notation_to_index(&from_spot) {
            if let Some(piece) = self.state.get_piece_at(index) {
                let from_row = chess_notation_utilities::convert_row(from_spot)?;
                let to_row = chess_notation_utilities::convert_row(to_spot)?;
                println!("from_row {:?} to_row: {:?}", from_row, to_row);
                //determine if its a pawn
                let bounds = chess_notation_utilities::get_bounds(to_spot)?;
                let (left_spot_opt, right_spot_opt, bottom_spot_opt, top_spot_opt) =
                    (bounds.left, bounds.right, bounds.bottom, bounds.top);
                if piece.get_unicode_val() == visual::WHITE_PAWN {
                    //determine coming from 2nd rank to 4th rank
                    if from_row == 6 && to_row == 4 {
                        //check left and right of to_spot for oposing pawn
                        if let Some(left_spot_array) = left_spot_opt {
                            if let Some(bottom_spot_array) = bottom_spot_opt {
                                let left_spot = std::str::from_utf8(&left_spot_array).unwrap();
                                let bottom_spot = std::str::from_utf8(&bottom_spot_array).unwrap();
                                if let Ok(index) =
                                    chess_notation_utilities::notation_to_index(left_spot)
                                {
                                    if let Some(piece) = self.state.get_piece_at(index) {
                                        if piece.get_unicode_val() == visual::BLACK_PAWN {
                                            //its there
                                            let enpassant_move =
                                                format!("{}-{}{}", left_spot, bottom_spot, to_spot);
                                            vec_en_passant_moves.push(enpassant_move);
                                        }
                                    }
                                }
                            }
                        }
                        if let Some(right_spot_array) = right_spot_opt {
                            if let Some(bottom_spot_array) = bottom_spot_opt {
                                let right_spot = std::str::from_utf8(&right_spot_array).unwrap();
                                let bottom_spot = std::str::from_utf8(&bottom_spot_array).unwrap();
                                if let Ok(index) =
                                    chess_notation_utilities::notation_to_index(right_spot)
                                {
                                    if let Some(piece) = self.state.get_piece_at(index) {
                                        if piece.get_unicode_val() == visual::BLACK_PAWN {
                                            //its there
                                            let enpassant_move = format!(
                                                "{}-{}{}",
                                                right_spot, bottom_spot, to_spot
                                            );
                                            vec_en_passant_moves.push(enpassant_move);
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if piece.get_unicode_val() == visual::BLACK_PAWN {
                    //determine coming from 2nd rank to 4th rank
                    if from_row == 1 && to_row == 3 {
                        //check left and right of to_spot for oposing pawn
                        if let Some(left_spot_array) = left_spot_opt {
                            if let Some(top_spot_array) = top_spot_opt {
                                let left_spot = std::str::from_utf8(&left_spot_array).unwrap();
                                let top_spot = std::str::from_utf8(&top_spot_array).unwrap();
                                if let Ok(index) =
                                    chess_notation_utilities::notation_to_index(left_spot)
                                {
                                    if let Some(piece) = self.state.get_piece_at(index) {
                                        if piece.get_unicode_val() == visual::WHITE_PAWN {
                                            //its there
                                            let enpassant_move =
                                                format!("{}-{}{}", left_spot, top_spot, to_spot);
                                            vec_en_passant_moves.push(enpassant_move);
                                        }
                                    }
                                }
                            }
                        }
                        if let Some(right_spot_array) = right_spot_opt {
                            if let Some(top_spot_array) = top_spot_opt {
                                let right_spot = std::str::from_utf8(&right_spot_array).unwrap();
                                let top_spot = std::str::from_utf8(&top_spot_array).unwrap();
                                if let Ok(index) =
                                    chess_notation_utilities::notation_to_index(right_spot)
                                {
                                    if let Some(piece) = self.state.get_piece_at(index) {
                                        if piece.get_unicode_val() == visual::WHITE_PAWN {
                                            //its there
                                            let enpassant_move =
                                                format!("{}-{}{}", right_spot, top_spot, to_spot);
                                            vec_en_passant_moves.push(enpassant_move);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if vec_en_passant_moves.len() > 0 {
            self.state.en_passant_enabled = Some(vec_en_passant_moves);
        } else {
            self.state.en_passant_enabled = None;
        }

        Ok(())
    }

    /// Plays a move given in long algebraic notation (see
    /// `chess_notation_utilities::parse_move`), e.g. `e2-e4`, `Ng1-f3`, `e4xd5`, `e7-e8=Q`, `O-O`.
    pub fn move_piece(
        &mut self,
        chess_move: &str,
        whos_turn: visual::PLAYER,
    ) -> Result<(), chess_errors::ChessErrors> {
        let invalid = || chess_errors::ChessErrors::InvalidNotation(chess_move.to_string());
        let parsed = match chess_notation_utilities::parse_move(chess_move)? {
            chess_notation_utilities::MoveNotation::Move(parsed) => parsed,
            castle => {
                // castling is the king moving two squares from its start square
                let rank = match whos_turn {
                    PLAYER::WHITE => '1',
                    PLAYER::BLACK => '8',
                };
                let to_file = match castle {
                    chess_notation_utilities::MoveNotation::CastleKingside => 'g',
                    _ => 'c',
                };
                chess_notation_utilities::ParsedMove {
                    piece: Some('k'),
                    from: format!("e{}", rank),
                    to: format!("{}{}", to_file, rank),
                    promotion: None,
                }
            }
        };
        let from_spot = parsed.from.as_str();
        let to_spot = parsed.to.as_str();
        let (from, to) =
            chess_notation_utilities::convert_move_notation_to_indexes(from_spot, to_spot)?;

        // a piece letter, if given, must match the piece on the from square
        if let Some(letter) = parsed.piece {
            let actual = self
                .state
                .get_piece_at(from)
                .map(|p| visual::get_piece_char_from_unicode(p.get_unicode_val()).to_ascii_lowercase());
            if actual != Some(letter) {
                return Err(invalid());
            }
        }

        let promotion_string = parsed.promotion.map(|c| c.to_string());
        let promotion = promotion_string.as_deref();
        let move_type = self.is_move_valid(from_spot, to_spot, whos_turn, promotion)?;
        // for the 50-move rule: pawn moves and captures reset the count
        let pawn_move = self.state.get_piece_at(from).map_or(false, |p| {
            p.get_unicode_val() == visual::WHITE_PAWN || p.get_unicode_val() == visual::BLACK_PAWN
        });
        let capture = self.state.get_piece_at(to).is_some()
            || matches!(move_type, visual::MoveType::Enpassant(_));
        self.check_en_passant(from_spot, to_spot, whos_turn)?;
        self.state.move_piece(from, to, promotion, move_type);
        if pawn_move || capture {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        Ok(())
    }
}

// }

// #[cfg(test)]
// mod tests {
//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     use super::*;

//     #[test]
//     fn test_promotion() {
//         //pawns reaching 8th rank can be promoted
//         let mut chess_game = Game::game_from_turn_history(&["a2-a4","b7-b5","a4-b5","f7-f5","b5-b6","b8-c6","b6-b7","f5-f4","a1-a7","g7-g6"]);
//         let good_move= chess_game.move_piece("b7-b8", chess_game.state.player_turn);
//         assert!(good_move.is_ok());
//         let bad_move= chess_game.move_piece("g7-g5pq", chess_game.state.player_turn);
//         assert!(bad_move.is_err());
//         let bad_move= chess_game.move_piece("a7-a8pq", chess_game.state.player_turn);
//         assert!(bad_move.is_err());
//     }
//     #[test]
//     fn test_enpassant1() {
//         //pawns reaching 8th rank can be promoted
//         let mut chess_game = Game::game_from_turn_history(&["a2-a4","b7-b5","a4-b5","f7-f5","b5-b6","b8-c6",
//         "b6-b7","f5-f4","a1-a7","g7-g6","d2-d4","h7-h5","d4-d5","h5-h4", "b2-b4","c6-a5", "b4-b5","c7-c5"]);
//         let good_move1= chess_game.move_piece("b5-c6", chess_game.state.player_turn);
//         assert!(good_move1.is_ok());
//         chess_game.state.player_turn = match chess_game.state.player_turn {
//             visual::PLAYER::WHITE => visual::PLAYER::BLACK,
//             visual::PLAYER::BLACK => visual::PLAYER::WHITE,
//         };
//         chess_game.move_piece("h8-h7", chess_game.state.player_turn);
//         chess_game.state.player_turn = match chess_game.state.player_turn {
//             visual::PLAYER::WHITE => visual::PLAYER::BLACK,
//             visual::PLAYER::BLACK => visual::PLAYER::WHITE,
//         };
//         chess_game.move_piece("g2-g4", chess_game.state.player_turn);
//         chess_game.state.player_turn = match chess_game.state.player_turn {
//             visual::PLAYER::WHITE => visual::PLAYER::BLACK,
//             visual::PLAYER::BLACK => visual::PLAYER::WHITE,
//         };
//         let good_move2= chess_game.move_piece("f4-g3", chess_game.state.player_turn);
//         assert!(good_move2.is_ok());
//     }

// #[test]
// fn test_enpassant2() {
//     //pawns reaching 8th rank can be promoted
//     let mut chess_game = Game::game_from_turn_history(&["a2-a4","b7-b5","a4-b5","f7-f5","b5-b6","b8-c6",
//     "b6-b7","f5-f4","a1-a7","g7-g6","d2-d4","h7-h5","d4-d5","h5-h4", "b2-b4","c6-a5", "b4-b5","c7-c5"]);
//     let good_move= chess_game.move_piece("d5-c6", chess_game.state.player_turn);
//     assert!(good_move.is_ok());
//     chess_game.move_piece("g2-g4", chess_game.state.player_turn);
//     let good_move= chess_game.move_piece("h4-g3", chess_game.state.player_turn);
//     assert!(good_move.is_ok());
// }
// }
