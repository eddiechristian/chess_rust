

use std::fmt;
use std::rc::Rc;
 
mod utilities;

const WHITE_PAWN: char  = '\u{2659}';
const WHITE_ROOK: char  = '\u{2656}';
const WHITE_KNIGHT: char  = '\u{2658}';
const WHITE_BISHOP: char  = '\u{2657}';

const BLACK_PAWN: char  = '\u{265F}';
const BLACK_ROOK: char  = '\u{265C}';
const BLACK_KNIGHT: char  = '\u{265E}';
const BLACK_BISHOP: char  = '\u{265D}';
const BLACK_QUEEN: char  = '\u{265B}';
const BLACK_KING: char  = '\u{265A}';

const WHITE_QUEEN: char  = '\u{2655}';
const WHITE_KING: char  = '\u{2654}';


enum PLAYER {
    WHITE,
    BLACK
}

trait GamePiece{
    fn get_unicode_val(&self)->char;
    fn move_forward_one(&self, pos: &str, state: &mut GameState )->Result<(),&'static str>{
        Ok(())
    }
    fn move_backward_one(&self, pos: &str)->Result<(),&'static str>{
        Err("invalid move")
    }
}

struct Pawn {
    unicode_val: char,
    player: PLAYER
}
impl fmt::Display for Pawn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|{}|", self.unicode_val)
    }
}

impl GamePiece for Pawn{
    fn get_unicode_val(&self) -> char{
       self.unicode_val
    }
}

struct Rook {
    unicode_val: char,
    player: PLAYER
}

impl fmt::Display for Rook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|{}|", self.unicode_val)
    }
}

impl GamePiece for Rook{
    fn get_unicode_val(&self) -> char{
       self.unicode_val
    }
}

struct Knight {
    unicode_val: char,
    player: PLAYER
}

impl fmt::Display for Knight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|{}|", self.unicode_val)
    }
}

impl GamePiece for Knight{
    fn get_unicode_val(&self) -> char{
       self.unicode_val
    }
    fn move_forward_one(&self, pos: &str, state: &mut GameState )->Result<(),&'static str>{
        Err("invalid move")
    }
}

struct Bishop {
    unicode_val: char,
    player: PLAYER
}

impl fmt::Display for Bishop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|{}|", self.unicode_val)
    }
}

impl GamePiece for Bishop{
    fn get_unicode_val(&self) -> char{
       self.unicode_val
    }
}

struct Queen {
    unicode_val: char,
    player: PLAYER
}

impl fmt::Display for Queen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|{}|", self.unicode_val)
    }
}

impl GamePiece for Queen{
    fn get_unicode_val(&self) -> char{
       self.unicode_val
    }
}

struct King {
    unicode_val: char,
    player: PLAYER
}

impl fmt::Display for King {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|{}|", self.unicode_val)
    }
}

impl GamePiece for King{
    fn get_unicode_val(&self) -> char{
       self.unicode_val
    }
}


struct GameState{
    state: Vec<Option<Rc<dyn GamePiece>>>,
    player_turn: PLAYER
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "========================\n")?;
        let mut ctr:u8 =0;
        for piece_opt in &self.state {
            ctr+=1;
            match piece_opt {
                Some(piece)=> {
                    write!(f, "|{}", piece.get_unicode_val())?
                }
                None => {
                    write!(f, "| ")?
                }
            }
            if ctr % 8 ==0 {
                write!(f, "|\n")?
            }
        }
        write!(f, "\n")
    }
}

impl Default for GameState {
    fn default() -> Self { 
        let black_rook1 = Rook {unicode_val: BLACK_ROOK,player: PLAYER::BLACK};
        let black_knight1 = Knight {unicode_val: BLACK_KNIGHT,player: PLAYER::BLACK};
        let black_bishop1 = Bishop {unicode_val: BLACK_BISHOP,player: PLAYER::BLACK};
        let black_queen = Queen {unicode_val: BLACK_QUEEN,player: PLAYER::BLACK};
        let black_king = Queen {unicode_val: BLACK_KING,player: PLAYER::BLACK};
        let black_bishop2 = Bishop {unicode_val: BLACK_BISHOP,player: PLAYER::BLACK};
        let black_knight2 = Knight {unicode_val: BLACK_KNIGHT,player: PLAYER::BLACK};
        let black_rook2 = Rook {unicode_val: BLACK_ROOK,player: PLAYER::BLACK};
        let black_pawn_rc = Rc::new(Pawn {unicode_val: BLACK_PAWN,player: PLAYER::BLACK});
    
        let white_pawn_rc = Rc::new(Pawn {unicode_val: WHITE_PAWN,player: PLAYER::WHITE});
        let white_rook1 = Rook {unicode_val: WHITE_ROOK,player: PLAYER::WHITE};
        let white_knight1 = Knight {unicode_val: WHITE_KNIGHT,player: PLAYER::WHITE};
        let white_bishop1 = Bishop {unicode_val: WHITE_BISHOP,player: PLAYER::WHITE};
        let white_queen = Queen {unicode_val: WHITE_QUEEN,player: PLAYER::WHITE};
        let white_king = Queen {unicode_val: WHITE_KING,player: PLAYER::WHITE};
        let white_bishop2 = Bishop {unicode_val: WHITE_BISHOP,player: PLAYER::WHITE};
        let white_knight2 = Knight {unicode_val: WHITE_KNIGHT,player: PLAYER::WHITE};
        let white_rook2 = Rook {unicode_val: WHITE_ROOK,player: PLAYER::WHITE};
    
        let mut pieces: Vec<Option<Rc<dyn GamePiece>>> = Vec::new();
        pieces.push(Some(Rc::new(black_rook1)));
        pieces.push(Some(Rc::new(black_knight1)));
        pieces.push(Some(Rc::new(black_bishop1)));
        pieces.push(Some(Rc::new(black_queen)));
        pieces.push(Some(Rc::new(black_king)));
        pieces.push(Some(Rc::new(black_bishop2)));
        pieces.push(Some(Rc::new(black_knight2)));
        pieces.push(Some(Rc::new(black_rook2)));
        for x in 0 ..8 {
            pieces.push(Some(black_pawn_rc.clone()));
        }
        for x in 0 ..32 {
            pieces.push(None);
        }
        for x in 0 ..8 {
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
        let  state = GameState {
            state: pieces,
            player_turn: PLAYER::WHITE
        };
        state
     }
}



impl GameState {
    
    fn move_piece(&mut self,from: &str, to: &str) -> Result<(),&'static str> {
        // This function does not validate whether or not the move is valid. It is done from calling functions
        if let Ok((from_id, to_id)) =  utilities::convert_move_notation_to_indexes(from, to){
            println!("from {:?} to {:?}", from_id, to_id);
            let value = std::mem::replace(&mut self.state[from_id as usize], None);
            std::mem::replace(&mut self.state[to_id as usize], value);
            return Ok(());
        }
        Err("invalid move")
    }
}
fn main() {
    let mut game_state = GameState::default();
    println!("{}",game_state);
    
    // game_state.move_piece("a8", "b8");
    
    // println!("{}",game_state);
   
    
    

}
