//for html
//https://stackoverflow.com/questions/26432492/chessboard-html5-only
//ai
//https://github.com/werner-duvaud/muzero-general

use std::{cell::RefCell, fmt};
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PLAYER {
    WHITE,
    BLACK,
}

pub trait GamePiece : std::fmt::Debug {
    fn get_unicode_val(&self) -> char;
    fn move_horizontal(&self, to_spot: &str, state: &GameState, delta_x: i8, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>;
    fn move_vertical(&self, to_spot: &str, state: &GameState, delta_y: i8, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>;
    fn move_diagonal(&self, to_spot: &str, state: &GameState, delta_y: i8, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>;
    fn move_knight(&self, to_spot: &str, state: &GameState, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>;
    fn get_player(&self) -> PLAYER;
    fn toggle_moved(&self);
    fn get_moved(&self) -> bool;
}
#[derive(Debug)]
pub struct Pawn {
    unicode_val: char,
    player: PLAYER,
    moved: RefCell<bool> ,
}
impl fmt::Display for Pawn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|{}|", self.unicode_val)
    }
}

impl GamePiece for Pawn {
    fn get_moved(&self) -> bool {
        *self.moved.borrow_mut() == true
    }
    fn toggle_moved(&self){
        self.moved.replace(true);
    }
    fn get_unicode_val(&self) -> char {
        self.unicode_val
    }
    fn get_player(&self) -> PLAYER{
        self.player
    }
    fn move_knight(&self, to_spot: &str, _state: &GameState, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
    fn  move_horizontal(&self, to_spot: &str, _state: &GameState, _delta_x: i8, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
    fn  move_vertical(&self, to_spot: &str, state: &GameState, delta_y: i8, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        if let Ok(index) = chess_notation_utilities::notation_to_index(&to_spot) {
            if  let Some(piece) = state.get_piece_at(index){
                if piece.get_player() != self.get_player(){
                     //pawns cannot attack forward
                    let msg = format!("{}",to_spot);
                    return Err(chess_errors::ChessErrors::PawnCantAttackForward(msg));
                }
            }
        }
        if delta_y.abs() > 2 {
            //pawns cannot move vert more than 2
            let msg = format!("{}",to_spot);
            return Err(chess_errors::ChessErrors::InvalidMove(msg));
        }
        if delta_y.abs() == 2 {
             //pawns cannot move vert more than 1, if they moved before
             if self.get_moved() {
                let msg = format!("{}",to_spot);
                return Err(chess_errors::ChessErrors::InvalidMove(msg));
             }
            //  if let Ok(index) = chess_notation_utilities::notation_to_index(&to_spot) {
            //     if  let Some(piece) = state.get_piece_at(index){
            //         if piece.get_moved() == true{
            //             let msg = format!("{}",to_spot);
            //             return Err(chess_errors::ChessErrors::InvalidMove(msg));
            //         }
            //     }
            // }
             
        }
        if self.get_player() == PLAYER::BLACK && delta_y > 0 {
            //black pawn annot move up
            let msg = format!("{}",to_spot);
            return Err(chess_errors::ChessErrors::InvalidMove(msg));
        } else if self.get_player() == PLAYER::WHITE && delta_y < 0 {
            //white pawn cannot move down
            let msg = format!("{}",to_spot);
            return Err(chess_errors::ChessErrors::InvalidMove(msg));
        }
        Ok(to_spot.to_string())
    }
    fn  move_diagonal(&self, to_spot: &str, state: &GameState, delta_y: i8, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        if self.get_player() == PLAYER::BLACK && delta_y > 0 {
            //black pawn annot move up
            let msg = format!("{}",to_spot);
            return Err(chess_errors::ChessErrors::InvalidMove(msg));
        } else if self.get_player() == PLAYER::WHITE && delta_y < 0 {
            //white pawn cannot move down
            let msg = format!("{}",to_spot);
            return Err(chess_errors::ChessErrors::InvalidMove(msg));
        }
        if let Ok(index) = chess_notation_utilities::notation_to_index(&to_spot) {
            if  state.get_piece_at(index).is_none() {
                let msg = format!("{}",to_spot);
                return Err(chess_errors::ChessErrors::PawnCanOnlyAttackDiagonal(msg));
            }
        }
        Ok(to_spot.to_string())
    }
}
#[derive(Debug)]
pub struct Rook {
    unicode_val: char,
    player: PLAYER,
    moved: RefCell<bool> ,
}
impl fmt::Display for Rook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|{}|", self.unicode_val)
    }
}

impl GamePiece for Rook {
    fn get_moved(&self) -> bool {
        self.moved == RefCell::new(true)
    }
    fn toggle_moved(&self){
        self.moved.replace(true);
    }
    fn get_unicode_val(&self) -> char {
        self.unicode_val
    }
    fn get_player(&self) -> PLAYER{
        self.player
    }
    fn move_knight(&self, to_spot: &str, _state: &GameState, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
    }
    fn  move_horizontal(&self, to_spot: &str, _state: &GameState, _delta_x: i8, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        if promotion.is_some() {
            let msg = format!("{}",to_spot);
            return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
        }
        Ok(to_spot.to_string())
    }
    fn  move_vertical(&self, to_spot: &str, _state: &GameState, _delta_y: i8, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        if promotion.is_some() {
            let msg = format!("{}",to_spot);
            return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
        }
        Ok(to_spot.to_string())
    }
    fn  move_diagonal(&self, to_spot: &str, _state: &GameState, _delta_y: i8, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
}
#[derive(Debug)]
pub struct Knight {
    unicode_val: char,
    player: PLAYER,
    moved: RefCell<bool> ,
}

impl fmt::Display for Knight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|{}|", self.unicode_val)
    }
}

impl GamePiece for Knight {
    fn get_moved(&self) -> bool {
        self.moved == RefCell::new(true)
    }
    fn toggle_moved(&self){
        self.moved.replace(true);
    }
    fn get_unicode_val(&self) -> char {
        self.unicode_val
    }
    fn get_player(&self) -> PLAYER{
        self.player
    }
    fn move_knight(&self, to_spot: &str, _state: &GameState, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        if promotion.is_some() {
            let msg = format!("{}",to_spot);
            return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
        }
        Ok(to_spot.to_string())
    }
    fn  move_horizontal(&self, to_spot: &str, _state: &GameState, _delta_x: i8, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
    fn  move_vertical(&self, to_spot: &str, _state: &GameState, _delta_y: i8, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
    fn  move_diagonal(&self, to_spot: &str, _state: &GameState, _delta_y: i8, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
}
#[derive(Debug)]
pub struct Bishop {
    unicode_val: char,
    player: PLAYER,
    moved: RefCell<bool> ,
}

impl fmt::Display for Bishop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|{}|", self.unicode_val)
    }
}

impl GamePiece for Bishop {
    fn get_moved(&self) -> bool {
        self.moved == RefCell::new(true)
    }
    fn toggle_moved(&self){
        self.moved.replace(true);
    }
    fn get_unicode_val(&self) -> char {
        self.unicode_val
    }
    fn get_player(&self) -> PLAYER{
        self.player
    }
    fn move_knight(&self, to_spot: &str, _state: &GameState, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
    fn  move_horizontal(&self, to_spot: &str, _state: &GameState, _delta_x: i8, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
    fn  move_vertical(&self, to_spot: &str, _state: &GameState, _delta_y: i8, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
    fn  move_diagonal(&self, to_spot: &str, _state: &GameState, _delta_y: i8, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        if promotion.is_some() {
            let msg = format!("{}",to_spot);
            return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
        }
        Ok(to_spot.to_string())
    }
}
#[derive(Debug)]
pub struct Queen {
    unicode_val: char,
    player: PLAYER,
    moved: RefCell<bool> ,
}

impl fmt::Display for Queen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|{}|", self.unicode_val)
    }
}

impl GamePiece for Queen {
    fn get_moved(&self) -> bool {
        self.moved == RefCell::new(true)
    }
    fn toggle_moved(&self){
        self.moved.replace(true);
    }
    fn get_unicode_val(&self) -> char {
        self.unicode_val
    }
    fn get_player(&self) -> PLAYER{
        self.player
    }
    fn move_knight(&self, to_spot: &str, _state: &GameState, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
    fn  move_horizontal(&self, to_spot: &str, _state: &GameState, _delta_x: i8, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        if promotion.is_some() {
            let msg = format!("{}",to_spot);
            return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
        }
        Ok(to_spot.to_string())
    }
    fn  move_vertical(&self, to_spot: &str, _state: &GameState, _delta_y: i8, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        if promotion.is_some() {
            let msg = format!("{}",to_spot);
            return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
        }
        Ok(to_spot.to_string())
    }
    fn  move_diagonal(&self, to_spot: &str, _state: &GameState, _delta_y: i8, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        if promotion.is_some() {
            let msg = format!("{}",to_spot);
            return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
        }
        Ok(to_spot.to_string())
    }
}
#[derive(Debug)]
pub struct King {
    unicode_val: char,
    player: PLAYER,
    moved: RefCell<bool> ,
}

impl fmt::Display for King {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|{}|", self.unicode_val)
    }
}

impl GamePiece for King {
    fn get_moved(&self) -> bool {
        self.moved == RefCell::new(true)
    }
    fn toggle_moved(&self){
        self.moved.replace(true);
    }
    fn get_unicode_val(&self) -> char {
        self.unicode_val
    }
    fn get_player(&self) -> PLAYER{
        self.player
    }
    fn move_knight(&self, to_spot: &str, _state: &GameState, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
    fn  move_horizontal(&self, to_spot: &str, _state: &GameState, _delta_x: i8, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        let msg = format!("{}",to_spot);
        return Err(chess_errors::ChessErrors::InvalidMove(msg));
    }
    fn  move_vertical(&self, to_spot: &str, _state: &GameState, _delta_y: i8, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        if promotion.is_some() {
            let msg = format!("{}",to_spot);
            return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
        }
        Ok(to_spot.to_string())
    }
    fn  move_diagonal(&self, to_spot: &str, _state: &GameState, _delta_y: i8, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>{
        if promotion.is_some() {
            let msg = format!("{}",to_spot);
            return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
        }
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
            moved: RefCell::new(false) ,
        };
        let black_knight1 = Knight {
            unicode_val: BLACK_KNIGHT,
            player: PLAYER::BLACK,
            moved: RefCell::new(false) ,
        };
        let black_bishop1 = Bishop {
            unicode_val: BLACK_BISHOP,
            player: PLAYER::BLACK,
            moved: RefCell::new(false) ,
        };
        let black_queen = Queen {
            unicode_val: BLACK_QUEEN,
            player: PLAYER::BLACK,
            moved: RefCell::new(false) ,
        };
        let black_king = Queen {
            unicode_val: BLACK_KING,
            player: PLAYER::BLACK,
            moved: RefCell::new(false) ,
        };
        let black_bishop2 = Bishop {
            unicode_val: BLACK_BISHOP,
            player: PLAYER::BLACK,
            moved: RefCell::new(false) ,
        };
        let black_knight2 = Knight {
            unicode_val: BLACK_KNIGHT,
            player: PLAYER::BLACK,
            moved: RefCell::new(false) ,
        };
        let black_rook2 = Rook {
            unicode_val: BLACK_ROOK,
            player: PLAYER::BLACK,
            moved: RefCell::new(false) ,
        };
        let white_rook1 = Rook {
            unicode_val: WHITE_ROOK,
            player: PLAYER::WHITE,
            moved: RefCell::new(false) ,
        };
        let white_knight1 = Knight {
            unicode_val: WHITE_KNIGHT,
            player: PLAYER::WHITE,
            moved: RefCell::new(false) ,
        };
        let white_bishop1 = Bishop {
            unicode_val: WHITE_BISHOP,
            player: PLAYER::WHITE,
            moved: RefCell::new(false) ,
        };
        let white_queen = Queen {
            unicode_val: WHITE_QUEEN,
            player: PLAYER::WHITE,
            moved: RefCell::new(false) ,
        };
        let white_king = Queen {
            unicode_val: WHITE_KING,
            player: PLAYER::WHITE,
            moved: RefCell::new(false) ,
        };
        let white_bishop2 = Bishop {
            unicode_val: WHITE_BISHOP,
            player: PLAYER::WHITE,
            moved: RefCell::new(false) ,
        };
        let white_knight2 = Knight {
            unicode_val: WHITE_KNIGHT,
            player: PLAYER::WHITE,
            moved: RefCell::new(false) ,
        };
        let white_rook2 = Rook {
            unicode_val: WHITE_ROOK,
            player: PLAYER::WHITE,
            moved: RefCell::new(false) ,
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
        for _x in 0..8 {
            let black_pawn = Pawn {
                unicode_val: BLACK_PAWN,
                player: PLAYER::BLACK,
                moved: RefCell::new(false) ,
            };
            pieces.push(Some(Rc::new(black_pawn)));
        }
        for _x in 0..32 {
            pieces.push(None);
        }
        for _x in 0..8 {
            let white_pawn = Pawn {
                unicode_val: WHITE_PAWN,
                player: PLAYER::WHITE,
                moved: RefCell::new(false) ,
            };
            pieces.push(Some(Rc::new(white_pawn)));
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
    pub fn move_piece(&mut self, from: usize, to: usize, promotion:  Option<&str>) {
        // This function does not validate whether or not the move is valid. It is done from calling functions
        let value = std::mem::replace(&mut self.state[from], None);
        println!("piece {:?} promotion {:?}",value.as_ref().unwrap() ,promotion);
        value.as_ref().unwrap().toggle_moved();
        println!("piece {:?}",value.as_ref().unwrap() );
        let  _ = std::mem::replace(&mut self.state[to], value);
    }

    pub fn get_piece_at(&self, pos: usize) -> Option<Rc<dyn GamePiece>> {
        if let Some(a) = self.state[pos].as_ref() {
            Some(a.clone())
        } else {
            None
        }
    }
}
