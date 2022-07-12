//for html
//https://stackoverflow.com/questions/26432492/chessboard-html5-only
//ai
//https://github.com/werner-duvaud/muzero-general

use std::{cell::RefCell, fmt};
use std::rc::Rc;

use crate::chess_notation_utilities;
use crate::chess_errors;

pub const WHITE_PAWN: char = '\u{2659}';
const WHITE_ROOK: char = '\u{2656}';
const WHITE_KNIGHT: char = '\u{2658}';
const WHITE_BISHOP: char = '\u{2657}';

pub const BLACK_PAWN: char = '\u{265F}';
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

#[derive(Debug)]
pub enum MoveType {
    Enpassant(usize),
    Castling,
    Regular,
    Promotion(char)
}

pub trait GamePiece : std::fmt::Debug {
    fn get_unicode_val(&self) -> char;
    fn move_horizontal(&self, to_spot: &str, state: &GameState, delta_x: i8, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>;
    fn move_vertical(&self, to_spot: &str, state: &GameState, delta_y: i8, promotion: Option<&str>) -> Result<(String,MoveType), chess_errors::ChessErrors>;
    fn move_diagonal(&self, to_spot: &str, state: &GameState, delta_y: i8, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>;
    fn move_knight(&self, to_spot: &str, state: &GameState, promotion: Option<&str>) -> Result<String, chess_errors::ChessErrors>;
    fn get_player(&self) -> PLAYER;
    fn toggle_moved(&self);
    fn get_moved(&self) -> bool;
    fn get_unvalidated_moves(&self, state: &GameState, spot: &str)-> Result<Vec<String>, chess_errors::ChessErrors>;
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
    fn get_unvalidated_moves(&self, state: &GameState, spot: &str)-> Result<Vec<String>, chess_errors::ChessErrors> {
        let mut unvalidated_moves = Vec::new();
        let bounds = chess_notation_utilities::get_bounds(spot)?;
        if let Ok(index) = chess_notation_utilities::notation_to_index(&spot) {
            if  let Some(piece) = state.get_piece_at(index){
                if piece.get_player() == PLAYER::WHITE{
                    let (top_opt, top_right_opt, top_left_opt) = (bounds.top, bounds.top_right_diag, bounds.top_left_diag);
                    if let Some(top_array) = top_opt {
                        let top = std::str::from_utf8(&top_array).unwrap();
                        let unvalidated_move = format!("{}-{}",spot,top);
                        unvalidated_moves.push(unvalidated_move);
                        let next_bounds = chess_notation_utilities::get_bounds(top)?;
                        if let Some(next_top_array) = next_bounds.top {
                            let next_top = std::str::from_utf8(&next_top_array).unwrap();
                            let unvalidated_move = format!("{}-{}",spot,next_top);
                            unvalidated_moves.push(unvalidated_move);
                        }
                    }
                    if let Some(top_right_array) = top_right_opt {
                        let top_right = std::str::from_utf8(&top_right_array).unwrap();
                        let unvalidated_move = format!("{}-{}",spot,top_right);
                        unvalidated_moves.push(unvalidated_move);
                    }
                    if let Some(top_left_array) = top_left_opt {
                        let top_left = std::str::from_utf8(&top_left_array).unwrap();
                        let unvalidated_move = format!("{}-{}",spot,top_left);
                        unvalidated_moves.push(unvalidated_move);
                    }
                } else if piece.get_player() == PLAYER::BLACK{
                    let (bottom_opt, bottom_right_opt, bottom_left_opt) = (bounds.bottom, bounds.bottom_right_diag, bounds.bottom_left_diag);
                    if let Some(bottom_array) = bottom_opt {
                        let bottom = std::str::from_utf8(&bottom_array).unwrap();
                        let unvalidated_move = format!("{}-{}",spot,bottom);
                        unvalidated_moves.push(unvalidated_move);
                        let next_bounds = chess_notation_utilities::get_bounds(bottom)?;
                        if let Some(next_bottom_array) = next_bounds.bottom {
                            let next_bottom = std::str::from_utf8(&next_bottom_array).unwrap();
                            let unvalidated_move = format!("{}-{}",spot,next_bottom);
                            unvalidated_moves.push(unvalidated_move);
                        }
                    }
                    if let Some(bottom_right_array) = bottom_right_opt {
                        let bottom_right = std::str::from_utf8(&bottom_right_array).unwrap();
                        let unvalidated_move = format!("{}-{}",spot,bottom_right);
                        unvalidated_moves.push(unvalidated_move);
                    }
                    if let Some(bottom_left_array) = bottom_left_opt {
                        let bottom_left = std::str::from_utf8(&bottom_left_array).unwrap();
                        let unvalidated_move = format!("{}-{}",spot,bottom_left);
                        unvalidated_moves.push(unvalidated_move);
                    }
                }
            }
        } 
        
        Ok((unvalidated_moves))
    }

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
    fn  move_vertical(&self, to_spot: &str, state: &GameState, delta_y: i8, promotion_opt: Option<&str>) -> Result<(String,MoveType), chess_errors::ChessErrors>{
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
        }
        if self.get_player() == PLAYER::BLACK && delta_y > 0 {
            //black pawn cannot move up
            let msg = format!("{}",to_spot);
            return Err(chess_errors::ChessErrors::InvalidMove(msg));
        } else if self.get_player() == PLAYER::WHITE && delta_y < 0 {
            //white pawn cannot move down
            let msg = format!("{}",to_spot);
            return Err(chess_errors::ChessErrors::InvalidMove(msg));
        }
        //check for promotion
        if let Ok(row) =chess_notation_utilities::convert_row(to_spot){
            if self.get_player() == PLAYER::BLACK {
                if row == 7 {
                    match promotion_opt {
                        None => {
                            return Ok((to_spot.to_string(),MoveType::Promotion(BLACK_QUEEN)))
                        },
                        Some(promotion) => {
                            match promotion {
                                "r" => {
                                    return Ok((to_spot.to_string(),MoveType::Promotion(BLACK_ROOK)));
                                },
                                "b" => {
                                    return Ok((to_spot.to_string(),MoveType::Promotion(BLACK_BISHOP)));
                                },
                                "k" => {
                                    return Ok((to_spot.to_string(),MoveType::Promotion(BLACK_KNIGHT)));
                                },
                                _ => {
                                    return Ok((to_spot.to_string(),MoveType::Promotion(BLACK_QUEEN)));
                                },
                            }
                        }
                    }
                }
            } else {
                if row == 0 {
                    match promotion_opt {
                        None => {
                            return Ok((to_spot.to_string(),MoveType::Promotion(WHITE_QUEEN)))
                        },
                        Some(promotion) => {
                            match promotion {
                                "r" => {
                                    return Ok((to_spot.to_string(),MoveType::Promotion(WHITE_ROOK)));
                                },
                                "b" => {
                                    return Ok((to_spot.to_string(),MoveType::Promotion(WHITE_BISHOP)));
                                },
                                "k" => {
                                    return Ok((to_spot.to_string(),MoveType::Promotion(WHITE_KNIGHT)));
                                },
                                _ => {
                                    return Ok((to_spot.to_string(),MoveType::Promotion(WHITE_QUEEN)));
                                },
                            }
                            
                        }
                    }
                }
            }   
        }
       
        Ok((to_spot.to_string(),MoveType::Regular))
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
    fn get_unvalidated_moves(&self, state: &GameState, spot: &str)-> Result<Vec<String>, chess_errors::ChessErrors>  {
        chess_notation_utilities::get_unvalidated_horiz_vert_moves(spot)
    }
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
    fn  move_vertical(&self, to_spot: &str, _state: &GameState, _delta_y: i8, promotion: Option<&str>) -> Result<(String,MoveType), chess_errors::ChessErrors>{
        if promotion.is_some() {
            let msg = format!("{}",to_spot);
            return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
        }
        Ok((to_spot.to_string(),MoveType::Regular))
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
    fn get_unvalidated_moves(&self, state: &GameState, spot: &str)-> Result<Vec<String>, chess_errors::ChessErrors> {
        let mut unvalidated_moves = Vec::new();
        let col_spot:u8  = (chess_notation_utilities::convert_col(spot)?) as u8;
        let row_spot:u8 = (chess_notation_utilities::convert_row(spot)?)as u8;
        let row_minus2 = match row_spot{
            2..=7 => Some(row_spot - 2),
            _ => None,
        };
        let row_minus1 = match row_spot{
            1..=7 => Some(row_spot - 1),
            _ => None,
        };
        let col_minus2 = match col_spot{
            2..=7 => Some(col_spot - 2),
            _ => None,
        };
        let col_minus1 = match col_spot{
            1..=7 => Some(col_spot - 1),
            _ => None,
        };
        let row_plus2 = match row_spot{
            0..=5 => Some(row_spot + 2),
            _ => None,
        };
        let row_plus1 = match row_spot{
            0..=6 => Some(row_spot + 1),
            _ => None,
        };
        let col_plus2 = match col_spot{
            0..=5 => Some(col_spot + 2),
            _ => None,
        };
        let col_plus1 = match col_spot{
            0..=6 => Some(col_spot + 1),
            _ => None,
        };
        //up 2 right 1
        if row_minus2.is_some() && col_plus1.is_some() {
            let up2rt1_index = row_minus2.unwrap() * 8 + col_plus1.unwrap();
            let mut unvalidated_move = format!("{}-{}",spot, chess_notation_utilities::index_to_spot(up2rt1_index as usize));
            unvalidated_moves.push(unvalidated_move);
        }
        //up 1 right 2
        if row_minus1.is_some() && col_plus2.is_some() {
            let up1rt2_index = row_minus1.unwrap() * 8 + col_plus2.unwrap();
            let mut unvalidated_move = format!("{}-{}",spot, chess_notation_utilities::index_to_spot(up1rt2_index as usize));
            unvalidated_moves.push(unvalidated_move);
        }
        //up 2 left 1
        if row_minus2.is_some() && col_minus1.is_some() {
            let up2lf1_index = row_minus2.unwrap() * 8 + col_minus1.unwrap();
            let mut unvalidated_move = format!("{}-{}",spot, chess_notation_utilities::index_to_spot(up2lf1_index as usize));
            unvalidated_moves.push(unvalidated_move);
        }
        //up 1 left 2
        if row_minus1.is_some() && col_minus2.is_some() {
            let up1lf2_index = row_minus1.unwrap() * 8 + col_minus2.unwrap();
            let mut unvalidated_move = format!("{}-{}",spot, chess_notation_utilities::index_to_spot(up1lf2_index as usize));
            unvalidated_moves.push(unvalidated_move);
        }
        //down 2 right 1
        if row_plus2.is_some() && col_plus1.is_some() {
            let dn2rt1_index = row_plus2.unwrap() * 8 + col_plus1.unwrap();
            let mut unvalidated_move = format!("{}-{}",spot, chess_notation_utilities::index_to_spot(dn2rt1_index as usize));
            unvalidated_moves.push(unvalidated_move);
        }
        //down 1 right 2
        if row_plus1.is_some() && col_plus2.is_some() {
            let dn1rt2_index = row_plus1.unwrap() * 8 + col_plus2.unwrap();
            let mut unvalidated_move = format!("{}-{}",spot, chess_notation_utilities::index_to_spot(dn1rt2_index as usize));
            unvalidated_moves.push(unvalidated_move);
        }
        //down 2 left 1
        if row_plus2.is_some() && col_minus1.is_some() {
            let dn2lf1_index = row_plus2.unwrap() * 8 + col_minus1.unwrap();
            let mut unvalidated_move = format!("{}-{}",spot, chess_notation_utilities::index_to_spot(dn2lf1_index as usize));
            unvalidated_moves.push(unvalidated_move);
        }
        //down 1 left 2
        if row_plus1.is_some() && col_minus2.is_some() {
            let dn1lf2_index = row_plus1.unwrap() * 8 + col_minus2.unwrap();
            let mut unvalidated_move = format!("{}-{}",spot, chess_notation_utilities::index_to_spot(dn1lf2_index as usize));
            unvalidated_moves.push(unvalidated_move);
        }
        Ok((unvalidated_moves))
    }
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
    fn  move_vertical(&self, to_spot: &str, _state: &GameState, _delta_y: i8, promotion: Option<&str>) -> Result<(String,MoveType), chess_errors::ChessErrors>{
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
    fn get_unvalidated_moves(&self, state: &GameState, spot: &str)-> Result<Vec<String>, chess_errors::ChessErrors> {
        chess_notation_utilities::get_unvalidated_diag_moves(spot)
    }
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
    fn  move_vertical(&self, to_spot: &str, _state: &GameState, _delta_y: i8, promotion: Option<&str>) -> Result<(String,MoveType), chess_errors::ChessErrors>{
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
    fn get_unvalidated_moves(&self, state: &GameState, spot: &str)-> Result<Vec<String>, chess_errors::ChessErrors> {
        let mut unvalidated_moves = chess_notation_utilities::get_unvalidated_diag_moves(spot)?;
        unvalidated_moves.append(&mut chess_notation_utilities::get_unvalidated_horiz_vert_moves(spot)?);
        Ok((unvalidated_moves))
    }
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
    fn  move_vertical(&self, to_spot: &str, _state: &GameState, _delta_y: i8, promotion: Option<&str>) -> Result<(String,MoveType), chess_errors::ChessErrors>{
        if promotion.is_some() {
            let msg = format!("{}",to_spot);
            return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
        }
        Ok((to_spot.to_string(),MoveType::Regular))
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
    fn get_unvalidated_moves(&self, state: &GameState, spot: &str)-> Result<Vec<String>, chess_errors::ChessErrors> {
        let mut unvalidated_moves = Vec::new();
        let bounds = chess_notation_utilities::get_bounds(spot)?;

        if let Some(top_right_array) = bounds.top_right_diag {
            let top_right = std::str::from_utf8(&top_right_array).unwrap();
            let unvalidated_move = format!("{}-{}",spot,top_right);
            unvalidated_moves.push(unvalidated_move);
        }
        if let Some(top_left_array) = bounds.top_left_diag {
            let top_left = std::str::from_utf8(&top_left_array).unwrap();
            let unvalidated_move = format!("{}-{}",spot,top_left);
            unvalidated_moves.push(unvalidated_move);
        }
        if let Some(bottom_left_array) = bounds.bottom_left_diag {
            let bottom_left = std::str::from_utf8(&bottom_left_array).unwrap();
            let unvalidated_move = format!("{}-{}",spot,bottom_left);
            unvalidated_moves.push(unvalidated_move);
        }
        if let Some(bottom_right_array) = bounds.bottom_right_diag {
            let bottom_right = std::str::from_utf8(&bottom_right_array).unwrap();
            let unvalidated_move = format!("{}-{}",spot,bottom_right);
            unvalidated_moves.push(unvalidated_move);
        }
        if let Some(bottom_array) = bounds.bottom {
            let bottom = std::str::from_utf8(&bottom_array).unwrap();
            let unvalidated_move = format!("{}-{}",spot,bottom);
            unvalidated_moves.push(unvalidated_move);
        }
        if let Some(top_array) = bounds.top {
            let top = std::str::from_utf8(&top_array).unwrap();
            let unvalidated_move = format!("{}-{}",spot,top);
            unvalidated_moves.push(unvalidated_move);
        }
        if let Some(left_array) = bounds.left {
            let left = std::str::from_utf8(&left_array).unwrap();
            let unvalidated_move = format!("{}-{}",spot,left);
            unvalidated_moves.push(unvalidated_move);
        }
        if let Some(right_array) = bounds.right {
            let right = std::str::from_utf8(&right_array).unwrap();
            let unvalidated_move = format!("{}-{}",spot,right);
            unvalidated_moves.push(unvalidated_move);
        }

        Ok((unvalidated_moves))
    }
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
    fn  move_vertical(&self, to_spot: &str, _state: &GameState, _delta_y: i8, promotion: Option<&str>) -> Result<(String,MoveType), chess_errors::ChessErrors>{
        if promotion.is_some() {
            let msg = format!("{}",to_spot);
            return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
        }
        Ok((to_spot.to_string(),MoveType::Regular))
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
    pub en_passant_enabled: Option<Vec<String>>,
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
        let black_king = King {
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
        let white_king = King {
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
            en_passant_enabled: None,
        };
        state
    }
}

impl GameState {
    pub fn get_unvalidated_moves(&self) -> Option<Vec<String>>{
        //let mut unvalidated_moves = Vec::new();
        for (index, piece_opt) in self.state.iter().enumerate(){
            if  let Some(piece) = piece_opt {
                let spot = chess_notation_utilities::index_to_spot(index);
                let moves = piece.get_unvalidated_moves(self, &spot);
                println!("spot: {:?} moves: {:?}",spot, moves);
            }
            
            
        }
        None
    }

    fn promotion_game_piece(&self, piece_char:char) -> Option<Rc<dyn GamePiece>>{
        match piece_char{
            (WHITE_QUEEN) => {
                let white_queen = Queen {
                                unicode_val: piece_char,
                                player: PLAYER::WHITE,
                                moved: RefCell::new(true) ,
                            };
                Some(Rc::new(white_queen))
            },
            (WHITE_ROOK) => {
                let white_rook = Rook {
                                unicode_val: piece_char,
                                player: PLAYER::WHITE,
                                moved: RefCell::new(true) ,
                            };
                Some(Rc::new(white_rook))
            },
            (WHITE_BISHOP) => {
                let white_bishop = Bishop {
                                unicode_val: piece_char,
                                player: PLAYER::WHITE,
                                moved: RefCell::new(true) ,
                            };
                Some(Rc::new(white_bishop))
            },
            (WHITE_KNIGHT) => {
                let white_knight = Knight {
                                unicode_val: piece_char,
                                player: PLAYER::WHITE,
                                moved: RefCell::new(true) ,
                            };
                Some(Rc::new(white_knight))
            },
            (BLACK_QUEEN) => {
                let black_queen = Queen {
                    unicode_val: piece_char,
                    player: PLAYER::BLACK,
                    moved: RefCell::new(true) ,
                };
                Some(Rc::new(black_queen))
            },
            (BLACK_ROOK) => {
                let black_rook = Rook {
                    unicode_val: piece_char,
                    player: PLAYER::BLACK,
                    moved: RefCell::new(true) ,
                };
                Some(Rc::new(black_rook))
            },
            (BLACK_KNIGHT) => {
                let black_knight = Knight {
                    unicode_val: piece_char,
                    player: PLAYER::BLACK,
                    moved: RefCell::new(true) ,
                };
                Some(Rc::new(black_knight))
            },
            (BLACK_BISHOP) => {
                let black_bishop = Bishop {
                    unicode_val: piece_char,
                    player: PLAYER::BLACK,
                    moved: RefCell::new(true) ,
                };
                Some(Rc::new(black_bishop))
            },
            _ => None
        }
        
    }
    
    pub fn move_piece(&mut self, from: usize, to: usize, promotion:  Option<&str>, move_type: MoveType ) {
        // This function does not validate whether or not the move is valid. It is done from calling functions
        let value = std::mem::replace(&mut self.state[from], None);
        if let MoveType::Enpassant(index) =  move_type{
            let _= std::mem::replace(&mut self.state[index], None);
            self.en_passant_enabled = None;
        }
        if let MoveType::Promotion(piece_char) =  move_type{
            let new_piece = self.promotion_game_piece(piece_char);
            if new_piece.is_some() {
                let _= std::mem::replace(&mut self.state[to], new_piece);
            }
        }else {
            value.as_ref().unwrap().toggle_moved();
            let  _ = std::mem::replace(&mut self.state[to], value);
        }
        
       
    }

    pub fn get_piece_at(&self, pos: usize) -> Option<Rc<dyn GamePiece>> {
        if let Some(a) = self.state[pos].as_ref() {
            Some(a.clone())
        } else {
            None
        }
    }
}
