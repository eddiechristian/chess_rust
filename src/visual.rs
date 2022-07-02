use std::fmt;
use std::rc::Rc;

use crate::chess_notation_utilities;
use crate::chess_errors;

const WHITE_PAWN: char = '\u{2659}';
const WHITE_ROOK: char = '\u{2656}';
const WHITE_KNIGHT: char = '\u{2658}';
const WHITE_BISHOP: char = '\u{2657}';

const BLACK_PAWN: char = '\u{265F}';
const BLACK_ROOK: char = '\u{265C}';
const BLACK_KNIGHT: char = '\u{265E}';
const BLACK_BISHOP: char = '\u{265D}';
const BLACK_QUEEN: char = '\u{265B}';
const BLACK_KING: char = '\u{265A}';

const WHITE_QUEEN: char = '\u{2655}';
const WHITE_KING: char = '\u{2654}';

#[derive(Copy, Clone, PartialEq)]
pub enum PLAYER {
    WHITE,
    BLACK,
}

pub trait GamePiece {
    fn get_unicode_val(&self) -> char;
    fn  move_down_one(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors>;
    fn  move_up_one(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors>;
    fn  move_left_one(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors>;
    fn  move_right_one(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors>;
    fn  move_diagonal(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors>;
    fn get_player(&self) -> PLAYER;
}

pub struct Pawn {
    unicode_val: char,
    player: PLAYER,
}
impl fmt::Display for Pawn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|{}|", self.unicode_val)
    }
}

impl GamePiece for Pawn {
    fn get_unicode_val(&self) -> char {
        self.unicode_val
    }
    fn get_player(&self) -> PLAYER{
        self.player
    }
    fn  move_left_one(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors> {
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
    fn  move_right_one(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors>{
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
    fn  move_down_one(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors> {
        //remember pawn can only attach diagonal
        //check that here
        if self.player == PLAYER::WHITE {
            //white pawns cannot move up
            let msg = format!("{}",to_spot);
            return Err(chess_errors::ChessErrors::InvalidMove(msg));
        }
        if let Ok(index) = chess_notation_utilities::notation_to_index(&to_spot) {
            if let Some(piece) = state.get_piece_at(index) {
                if piece.get_player() != self.get_player(){
                    let msg = format!("{}",to_spot);
                    return Err(chess_errors::ChessErrors::PawnCantAttackForward(msg));
                }
            }
        }
        Ok((to_spot.to_string()))
    }
    fn  move_up_one(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors> {
        if self.player == PLAYER::BLACK {
            //black pawns cannot move up
            let msg = format!("{}",to_spot);
            return Err(chess_errors::ChessErrors::InvalidMove(msg));
        }
        if let Ok(index) = chess_notation_utilities::notation_to_index(&to_spot) {
            if let Some(piece) = state.get_piece_at(index) {
                if piece.get_player() != self.get_player(){
                    let msg = format!("{}",to_spot);
                    return Err(chess_errors::ChessErrors::PawnCantAttackForward(msg));
                }
            }
        }
        Ok(to_spot.to_string())
    }
    fn  move_diagonal(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors>{
        if let Ok(index) = chess_notation_utilities::notation_to_index(&to_spot) {
            if  state.get_piece_at(index).is_none() {
                let msg = format!("{}",to_spot);
                return Err(chess_errors::ChessErrors::PawnCanOnlyAttackDiagonal(msg));
            }
        }
        Ok(to_spot.to_string())
    }
}

pub struct Rook {
    unicode_val: char,
    player: PLAYER,
}
impl fmt::Display for Rook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|{}|", self.unicode_val)
    }
}

impl GamePiece for Rook {
    fn get_unicode_val(&self) -> char {
        self.unicode_val
    }
    fn get_player(&self) -> PLAYER{
        self.player
    }
    fn  move_left_one(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors> {
        panic!();
    }
    fn  move_right_one(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors>{
        panic!();
    }
    fn  move_down_one(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors> {
         panic!()
    }
    fn  move_up_one(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors> {
         panic!()
    }
    fn  move_diagonal(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors>{
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
}

pub struct Knight {
    unicode_val: char,
    player: PLAYER,
}

impl fmt::Display for Knight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|{}|", self.unicode_val)
    }
}

impl GamePiece for Knight {
    fn get_unicode_val(&self) -> char {
        self.unicode_val
    }
    fn get_player(&self) -> PLAYER{
        self.player
    }
    fn  move_left_one(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors> {
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
    fn  move_right_one(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors>{
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
    fn  move_down_one(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors> {
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
    fn  move_up_one(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors> {
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
    fn  move_diagonal(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors>{
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
}

pub struct Bishop {
    unicode_val: char,
    player: PLAYER,
}

impl fmt::Display for Bishop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|{}|", self.unicode_val)
    }
}

impl GamePiece for Bishop {
    fn get_unicode_val(&self) -> char {
        self.unicode_val
    }
    fn get_player(&self) -> PLAYER{
        self.player
    }
    fn  move_left_one(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors> {
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
    fn  move_right_one(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors>{
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
    fn  move_down_one(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors> {
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
    fn  move_up_one(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors> {
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
    fn  move_diagonal(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors>{
        Ok(to_spot.to_string())
    }
}

pub struct Queen {
    unicode_val: char,
    player: PLAYER,
}

impl fmt::Display for Queen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|{}|", self.unicode_val)
    }
}

impl GamePiece for Queen {
    fn get_unicode_val(&self) -> char {
        self.unicode_val
    }
    fn get_player(&self) -> PLAYER{
        self.player
    }
    fn  move_left_one(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors> {
        panic!();
    }
    fn  move_right_one(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors>{
        panic!();
    }
    fn  move_down_one(&self, pos: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors> {
         panic!()
    }
    fn  move_up_one(&self, pos: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors> {
         panic!()
    }
    fn  move_diagonal(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors>{
        Ok(to_spot.to_string())
    }
}

pub struct King {
    unicode_val: char,
    player: PLAYER,
}

impl fmt::Display for King {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|{}|", self.unicode_val)
    }
}

impl GamePiece for King {
    fn get_unicode_val(&self) -> char {
        self.unicode_val
    }
    fn get_player(&self) -> PLAYER{
        self.player
    }
    fn  move_left_one(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors> {
        panic!();
    }
    fn  move_right_one(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors>{
        panic!();
    }
    fn  move_down_one(&self, pos: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors> {
         panic!()
    }
    fn  move_up_one(&self, pos: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors> {
         panic!()
    }
    fn  move_diagonal(&self, to_spot: &str, state: &GameState) -> Result<String, chess_errors::ChessErrors>{
        Ok(to_spot.to_string())
    }
}

pub struct GameState {
    pub state: Vec<Option<Rc<dyn GamePiece>>>,
    pub player_turn: PLAYER,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut row: u8 = 8;
        let mut ctr: u8 = 0;
        write!(f, "8")?;
        for piece_opt in &self.state {
            match piece_opt {
                Some(piece) => write!(f, "|{}", piece.get_unicode_val())?,
                None => write!(f, "| ")?,
            }
            ctr += 1;
            if ctr % 8 ==0 {
                row -= 1;
                if row != 0 {
                    write!(f, "|\n{}", row)?;
                }
            }
            
        }
        write!(f, "\n  a b c d e f g h\n")
    }
}

impl Default for GameState {
    fn default() -> Self {
        let black_rook1 = Rook {
            unicode_val: BLACK_ROOK,
            player: PLAYER::BLACK,
        };
        let black_knight1 = Knight {
            unicode_val: BLACK_KNIGHT,
            player: PLAYER::BLACK,
        };
        let black_bishop1 = Bishop {
            unicode_val: BLACK_BISHOP,
            player: PLAYER::BLACK,
        };
        let black_queen = Queen {
            unicode_val: BLACK_QUEEN,
            player: PLAYER::BLACK,
        };
        let black_king = Queen {
            unicode_val: BLACK_KING,
            player: PLAYER::BLACK,
        };
        let black_bishop2 = Bishop {
            unicode_val: BLACK_BISHOP,
            player: PLAYER::BLACK,
        };
        let black_knight2 = Knight {
            unicode_val: BLACK_KNIGHT,
            player: PLAYER::BLACK,
        };
        let black_rook2 = Rook {
            unicode_val: BLACK_ROOK,
            player: PLAYER::BLACK,
        };
        let black_pawn_rc = Rc::new(Pawn {
            unicode_val: BLACK_PAWN,
            player: PLAYER::BLACK,
        });

        let white_pawn_rc = Rc::new(Pawn {
            unicode_val: WHITE_PAWN,
            player: PLAYER::WHITE,
        });
        let white_rook1 = Rook {
            unicode_val: WHITE_ROOK,
            player: PLAYER::WHITE,
        };
        let white_knight1 = Knight {
            unicode_val: WHITE_KNIGHT,
            player: PLAYER::WHITE,
        };
        let white_bishop1 = Bishop {
            unicode_val: WHITE_BISHOP,
            player: PLAYER::WHITE,
        };
        let white_queen = Queen {
            unicode_val: WHITE_QUEEN,
            player: PLAYER::WHITE,
        };
        let white_king = Queen {
            unicode_val: WHITE_KING,
            player: PLAYER::WHITE,
        };
        let white_bishop2 = Bishop {
            unicode_val: WHITE_BISHOP,
            player: PLAYER::WHITE,
        };
        let white_knight2 = Knight {
            unicode_val: WHITE_KNIGHT,
            player: PLAYER::WHITE,
        };
        let white_rook2 = Rook {
            unicode_val: WHITE_ROOK,
            player: PLAYER::WHITE,
        };

        let mut pieces: Vec<Option<Rc<dyn GamePiece>>> = Vec::new();
        pieces.push(Some(Rc::new(black_rook1)));
        pieces.push(Some(Rc::new(black_knight1)));
        pieces.push(Some(Rc::new(black_bishop1)));
        pieces.push(Some(Rc::new(black_queen)));
        pieces.push(Some(Rc::new(black_king)));
        pieces.push(Some(Rc::new(black_bishop2)));
        pieces.push(Some(Rc::new(black_knight2)));
        pieces.push(Some(Rc::new(black_rook2)));
        for x in 0..8 {
            pieces.push(Some(black_pawn_rc.clone()));
        }
        for x in 0..32 {
            pieces.push(None);
        }
        for x in 0..8 {
            pieces.push(Some(white_pawn_rc.clone()));
        }
        pieces.push(Some(Rc::new(white_rook1)));
        pieces.push(Some(Rc::new(white_knight1)));
        pieces.push(Some(Rc::new(white_bishop1)));
        pieces.push(Some(Rc::new(white_queen)));
        pieces.push(Some(Rc::new(white_king)));
        pieces.push(Some(Rc::new(white_bishop2)));
        pieces.push(Some(Rc::new(white_knight2)));
        pieces.push(Some(Rc::new(white_rook2)));
        let state = GameState {
            state: pieces,
            player_turn: PLAYER::WHITE,
        };
        state
    }
}

impl GameState {
    pub fn move_piece(&mut self, from: usize, to: usize) {
        // This function does not validate whether or not the move is valid. It is done from calling functions
        let value = std::mem::replace(&mut self.state[from], None);
        std::mem::replace(&mut self.state[to], value);
    }

    pub fn get_piece_at(&self, pos: usize) -> Option<Rc<dyn GamePiece>> {
        let a = self.state[pos].as_ref();
        if let Some(a) = self.state[pos].as_ref() {
            Some(a.clone())
        } else {
            None
        }
    }
}
