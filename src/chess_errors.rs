
use std::fmt;

use std::error::Error;
use std::str::Utf8Error;
#[derive(Debug)]
pub enum ChessErrors {
    InvalidNotation(String),
    WrongPlayer(String),
    NoPiece(String),
    PlayerPieceAlreadyThere(String),
    PawnCantAttackForward(String),
    InvalidMove(String),
    Utf8Error
}

impl From<Utf8Error> for ChessErrors {
    fn from(error: Utf8Error) -> Self {
        ChessErrors::Utf8Error
    }
}

//Utf8Error

impl Error for ChessErrors {}

impl fmt::Display for ChessErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            ChessErrors::InvalidNotation(x) => {
                write!(f, "{} is invalid chess notation", x)
            }
            ChessErrors::WrongPlayer(x) => {
                write!(f, "wrong player at {}", x)
            }
            ChessErrors::NoPiece(x) => {
                write!(f, "no piece at {}", x)
            }
            ChessErrors::PlayerPieceAlreadyThere(x) => {
                write!(f, "you have a  piece at {}", x)
            }
            ChessErrors::PawnCantAttackForward(x) => {
                write!(f, "pawn cant attack piece at {}", x)
            }
            ChessErrors::InvalidMove(x) => {
                write!(f, "piece cannot move to {}", x)
            }
            _ => {
                write!(f, "ddddd")
            }
        }
    }
}

pub fn try_error(value: i64) -> Result<(), ChessErrors> {
    let a: [u8; 2] = [97, 97];
    let b = std::str::from_utf8(&a).unwrap();
    let e = ChessErrors::InvalidNotation(b.to_string());
    Err(e)
}
