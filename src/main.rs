use std::str;
use std::fmt;

const WHITE_PAWN: char  = '\u{2659}';

const BLACK_PAWN: char  = '\u{265F}';
const BLACK_ROOK: char  = '\u{265C}';
const BLACK_KNIGHT: char  = '\u{265E}';
const BLACK_BISHOP: char  = '\u{265D}';
const BLACK_QUEEN: char  = '\u{265B}';
const BLACK_KING: char  = '\u{265A}';


enum PLAYER {
    WHITE,
    BLACK
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
    fn print_me(&self) -> char{
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
    fn print_me(&self) -> char{
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
    fn print_me(&self) -> char{
       self.unicode_val
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
    fn print_me(&self) -> char{
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
    fn print_me(&self) -> char{
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
    fn print_me(&self) -> char{
       self.unicode_val
    }
}

 trait GamePiece{
     fn print_me(&self)->char;
 }

 

struct GameState{
    state: Vec<Option<Box<dyn GamePiece>>>,
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
                    write!(f, "|{}", piece.print_me())?
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

fn main() {
    let black_rook1 = Rook {unicode_val: BLACK_ROOK,player: PLAYER::BLACK};
    let black_knight1 = Knight {unicode_val: BLACK_KNIGHT,player: PLAYER::BLACK};
    let black_bishop1 = Bishop {unicode_val: BLACK_BISHOP,player: PLAYER::BLACK};
    let black_queen = Queen {unicode_val: BLACK_QUEEN,player: PLAYER::BLACK};
    let black_king = Queen {unicode_val: BLACK_KING,player: PLAYER::BLACK};
    let black_bishop2 = Bishop {unicode_val: BLACK_BISHOP,player: PLAYER::BLACK};
    let black_knight2 = Knight {unicode_val: BLACK_KNIGHT,player: PLAYER::BLACK};
    let black_rook2 = Rook {unicode_val: BLACK_ROOK,player: PLAYER::BLACK};
    let black_pawn = Pawn {unicode_val: BLACK_PAWN,player: PLAYER::BLACK};
    
    let mut pieces: Vec<Option<Box<dyn GamePiece>>> = Vec::new();
    pieces.push(Some(Box::new(black_rook1)));
    pieces.push(Some(Box::new(black_knight1)));
    pieces.push(Some(Box::new(black_bishop1)));
    pieces.push(Some(Box::new(black_queen)));
    pieces.push(Some(Box::new(black_king)));
    pieces.push(Some(Box::new(black_bishop2)));
    pieces.push(Some(Box::new(black_knight2)));
    pieces.push(Some(Box::new(black_rook2)));
    for x in 0 ..9 {
        pieces.push(Some(Box::new(black_pawn)));
    }
    for x in 0 ..33 {
        pieces.push(None);
    }
    let mut state = GameState {
        state: pieces,
        player_turn: PLAYER::WHITE
    };
    println!("{}",state);
}
