use crate::chess_errors;
use crate::visual;
use core::f64::RADIX;
use std::convert::TryInto;
use std::fmt;

pub struct bounds {
    pub top: Option<[u8; 2]>,
    pub bottom: Option<[u8; 2]>,
    pub left: Option<[u8; 2]>,
    pub right: Option<[u8; 2]>,
    pub top_left_diag: Option<[u8; 2]>,
    pub top_right_diag: Option<[u8; 2]>,
    pub bottom_left_diag: Option<[u8; 2]>,
    pub bottom_right_diag: Option<[u8; 2]>,
}

impl fmt::Display for bounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(c) = self.top {
            let a = std::str::from_utf8(&c).unwrap();
            write!(f, "top: {}\n", a)?;
        } else {
            write!(f, "top: None\n")?;
        }
        if let Some(c) = self.bottom {
            let a = std::str::from_utf8(&c).unwrap();
            write!(f, "bottom: {}\n", a)?;
        } else {
            write!(f, "bottom: None\n")?;
        }
        if let Some(c) = self.left {
            let a = std::str::from_utf8(&c).unwrap();
            write!(f, "left: {}\n", a)?;
        } else {
            write!(f, "left: None\n")?;
        }
        if let Some(c) = self.right {
            let a = std::str::from_utf8(&c).unwrap();
            write!(f, "right: {}\n", a)?;
        } else {
            write!(f, "right: None\n")?;
        }
        write!(f, "\n")
    }
}

pub fn minus_one_col(the_col: char) -> Option<char> {
    match the_col {
        'b' => Some('a'),
        'c' => Some('b'),
        'd' => Some('c'),
        'e' => Some('d'),
        'f' => Some('e'),
        'g' => Some('f'),
        'h' => Some('g'),
        _ => None,
    }
}

pub fn plus_one_col(the_col: char) -> Option<char> {
    match the_col {
        'a' => Some('b'),
        'b' => Some('c'),
        'c' => Some('d'),
        'd' => Some('e'),
        'e' => Some('f'),
        'f' => Some('g'),
        'g' => Some('h'),
        _ => None,
    }
}

pub fn check_for_valid_notation(spot: &str) -> Result<bool, chess_errors::ChessErrors> {
    match &spot.chars().nth(0).unwrap() {
        'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' => match &spot.chars().nth(1).unwrap() {
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => Ok(true),
            _ => Err(chess_errors::ChessErrors::InvalidNotation(spot.to_string())),
        },
        _ => Err(chess_errors::ChessErrors::InvalidNotation(spot.to_string())),
    }
}

pub fn get_bounds(spot: &str) -> Result<bounds, chess_errors::ChessErrors> {
    check_for_valid_notation(spot)?;
    let top = {
        match &spot.chars().nth(1).unwrap() {
            '8' => None,
            row_char @ '1'
            | row_char @ '2'
            | row_char @ '3'
            | row_char @ '4'
            | row_char @ '5'
            | row_char @ '6'
            | row_char @ '7' => {
                if let Some(first_char) = spot.chars().nth(0) {
                    let mut row = row_char.to_digit(10).unwrap();
                    row += 1;
                    let y = format!("{}{}", first_char, row);
                    Some(y.as_bytes().try_into().unwrap())
                } else {
                    None
                }
            }
            _ => None,
        }
    };

    let bottom = {
        match &spot.chars().nth(1).unwrap() {
            '1' => None,
            row_char @ '8'
            | row_char @ '7'
            | row_char @ '6'
            | row_char @ '5'
            | row_char @ '4'
            | row_char @ '3'
            | row_char @ '2' => {
                if let Some(first_char) = spot.chars().nth(0) {
                    let mut row = row_char.to_digit(10).unwrap();
                    row -= 1;
                    let y = format!("{}{}", first_char, row);
                    Some(y.as_bytes().try_into().unwrap())
                } else {
                    None
                }
            }
            _ => None,
        }
    };

    let left = {
        match &spot.chars().nth(0).unwrap() {
            col_char @ 'b'
            | col_char @ 'c'
            | col_char @ 'd'
            | col_char @ 'e'
            | col_char @ 'f'
            | col_char @ 'g'
            | col_char @ 'h' => {
                if let Some(second_char) = spot.chars().nth(1) {
                    let y = format!("{}{}", minus_one_col(*col_char).unwrap(), second_char);
                    Some(y.as_bytes().try_into().unwrap())
                } else {
                    None
                }
            }
            _ => None,
        }
    };

    let right = {
        match &spot.chars().nth(0).unwrap() {
            col_char @ 'a'
            | col_char @ 'b'
            | col_char @ 'c'
            | col_char @ 'd'
            | col_char @ 'e'
            | col_char @ 'f'
            | col_char @ 'g' => {
                if let Some(second_char) = spot.chars().nth(1) {
                    let y = format!("{}{}", plus_one_col(*col_char).unwrap(), second_char);
                    Some(y.as_bytes().try_into().unwrap())
                } else {
                    None
                }
            }
            _ => None,
        }
    };

    let (top_left_diag, top_right_diag, bottom_left_diag, bottom_right_diag) =
        (None, None, None, None);

    let bounds = bounds {
        top: top,
        bottom: bottom,
        left: left,
        right: right,
        top_left_diag: top_left_diag,
        top_right_diag: top_right_diag,
        bottom_left_diag: bottom_left_diag,
        bottom_right_diag: bottom_right_diag,
    };
   Ok(bounds)
}

pub fn convert_col(spot: &str) -> Result<usize, &'static str> {
    let col = match spot.chars().nth(0) {
        Some(first_char) => match first_char {
            'a' => Ok(0),
            'b' => Ok(1),
            'c' => Ok(2),
            'd' => Ok(3),
            'e' => Ok(4),
            'f' => Ok(5),
            'g' => Ok(6),
            'h' => Ok(7),
            _ => Err("not valid move notation"),
        },
        None => Err("not valid move notation"),
    }?;

    Ok(col)
}

pub fn convert_row(spot: &str) -> Result<usize, &'static str> {
    let row = match spot.chars().nth(1) {
        Some(first_char) => match first_char {
            '8' => Ok(0),
            '7' => Ok(1),
            '6' => Ok(2),
            '5' => Ok(3),
            '4' => Ok(4),
            '3' => Ok(5),
            '2' => Ok(6),
            '1' => Ok(7),
            _ => Err("not valid move notation"),
        },
        None => Err("not valid move notation"),
    }?;

    Ok(row)
}

pub fn notation_to_index(spot: &str) -> Result<usize, &'static str> {
    let col = convert_col(spot)?;
    let row = convert_row(spot)?;
    let index = (row * 8) + col;
    Ok(index)
}

pub fn convert_move_notation_to_indexes(
    from_spot: &str,
    to_spot: &str,
) -> Result<(usize, usize), &'static str> {
    let from_index = notation_to_index(from_spot)?;
    let to_index = notation_to_index(to_spot)?;

    Ok((from_index, to_index))
}
