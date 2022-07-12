use crate::chess_errors;

use std::convert::TryInto;
use std::fmt;

pub struct Bounds {
    pub top: Option<[u8; 2]>,
    pub bottom: Option<[u8; 2]>,
    pub left: Option<[u8; 2]>,
    pub right: Option<[u8; 2]>,
    pub top_left_diag: Option<[u8; 2]>,
    pub top_right_diag: Option<[u8; 2]>,
    pub bottom_left_diag: Option<[u8; 2]>,
    pub bottom_right_diag: Option<[u8; 2]>,
}
#[derive(Copy, Clone, PartialEq)]
pub struct Point {
    pub x: u8,
    pub y: u8
}

impl fmt::Display for Bounds {
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
        if let Some(c) = self.top_left_diag {
            let a = std::str::from_utf8(&c).unwrap();
            write!(f, "top_left_diag: {}\n", a)?;
        } else {
            write!(f, "top_left_diag: None\n")?;
        }
        if let Some(c) = self.top_right_diag {
            let a = std::str::from_utf8(&c).unwrap();
            write!(f, "top_right_diag: {}\n", a)?;
        } else {
            write!(f, "top_right_diag: None\n")?;
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

pub fn get_unvalidated_diag_moves(spot: &str)->Result<Vec<String>, chess_errors::ChessErrors> {
    let mut right_top_spots = Vec::new();
    let mut right_bottom_spots = Vec::new();
    let mut left_bottom_spots = Vec::new();
    let mut left_top_spots = Vec::new();

    let mut right_top_pos_opt = Some(spot.to_string());
                
    loop {
            let bounds = get_bounds(&right_top_pos_opt.unwrap())?;
            if let Some(right_top_pos_array) = bounds.top_right_diag{
                let right_top_pos = std::str::from_utf8(&right_top_pos_array).unwrap();
                let value = format!("{}-{}",spot,right_top_pos);
                right_top_spots.push(value);
                right_top_pos_opt = Some(right_top_pos.to_string());
            } else {
                break;
            }

    }
    let mut right_bottom_pos_opt = Some(spot.to_string());
    loop {
            let bounds = get_bounds(&right_bottom_pos_opt.unwrap())?;
            if let Some(right_bottom_pos_array) = bounds.bottom_right_diag{
                let right_bottom_pos = std::str::from_utf8(&right_bottom_pos_array).unwrap();
                let value = format!("{}-{}",spot,right_bottom_pos);
                right_bottom_spots.push(value);
                right_bottom_pos_opt = Some(right_bottom_pos.to_string());
            } else {
                break;
            }

    }
    
    let mut left_bottom_pos_opt = Some(spot.to_string());
    loop {
            let bounds = get_bounds(&left_bottom_pos_opt.unwrap())?;
            if let Some(left_bottom_pos_array) = bounds.bottom_left_diag{
                let left_bottom_pos = std::str::from_utf8(&left_bottom_pos_array).unwrap();
                let value = format!("{}-{}",spot,left_bottom_pos);
                left_bottom_spots.push(value);
                left_bottom_pos_opt = Some(left_bottom_pos.to_string());
            } else {
                break;
            }

    }
    
    let mut left_top_pos_opt = Some(spot.to_string());
    loop {
            let bounds = get_bounds(&left_top_pos_opt.unwrap())?;
            if let Some(left_top_pos_array) = bounds.top_left_diag{
                let left_top_pos = std::str::from_utf8(&left_top_pos_array).unwrap();
                let value = format!("{}-{}",spot,left_top_pos);
                left_top_spots.push(value);
                left_top_pos_opt = Some(left_top_pos.to_string());
            } else {
                break;
            }

    }

    right_top_spots.append(&mut right_bottom_spots);
    right_top_spots.append(&mut left_bottom_spots);
    right_top_spots.append(&mut left_top_spots);
    Ok(right_top_spots)
}


pub fn get_unvalidated_horiz_vert_moves(spot: &str)->Result<Vec<String>, chess_errors::ChessErrors> {
    let mut right_spots = Vec::new();
    let mut top_spots = Vec::new();
    let mut bottom_spots = Vec::new();
    let mut left_spots = Vec::new();

    let mut right_pos_opt = Some(spot.to_string());
    loop {
            let bounds = get_bounds(&right_pos_opt.unwrap())?;
            if let Some(right_pos_array) = bounds.right{
                let right_pos = std::str::from_utf8(&right_pos_array).unwrap();
                let value = format!("{}-{}",spot,right_pos);
                right_spots.push(value);
                right_pos_opt = Some(right_pos.to_string());
            } else {
                break;
            }

    }
    
    let mut top_pos_opt = Some(spot.to_string());
    loop {
            let bounds = get_bounds(&top_pos_opt.unwrap())?;
            if let Some(top_pos_array) = bounds.top{
                let top_pos = std::str::from_utf8(&top_pos_array).unwrap();
                let value = format!("{}-{}",spot,top_pos);
                top_spots.push(value);
                top_pos_opt = Some(top_pos.to_string());
            } else {
                break;
            }

    }
        
    let mut bottom_pos_opt = Some(spot.to_string());
    loop {
            let bounds = get_bounds(&bottom_pos_opt.unwrap())?;
            if let Some(bottom_pos_array) = bounds.bottom{
                let bottom_pos = std::str::from_utf8(&bottom_pos_array).unwrap();
                let value = format!("{}-{}",spot,bottom_pos);
                bottom_spots.push(value);
                bottom_pos_opt = Some(bottom_pos.to_string());
            } else {
                break;
            }

    }
    
    let mut left_pos_opt = Some(spot.to_string());
    loop {
            let bounds = get_bounds(&left_pos_opt.unwrap())?;
            if let Some(left_pos_array) = bounds.left{
                let left_pos = std::str::from_utf8(&left_pos_array).unwrap();
                let value = format!("{}-{}",spot,left_pos);
                left_spots.push(value);
                left_pos_opt = Some(left_pos.to_string());
            } else {
                break;
            }

    }

    right_spots.append(&mut top_spots);
    right_spots.append(&mut bottom_spots);
    right_spots.append(&mut left_spots);
    Ok(right_spots)
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

pub fn get_bounds(spot: &str) -> Result<Bounds, chess_errors::ChessErrors> {
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
    let bottom_right_diag = {
        match &spot.chars().nth(0).unwrap() {
            col_char @ 'a'
            | col_char @ 'b'
            | col_char @ 'c'
            | col_char @ 'd'
            | col_char @ 'e'
            | col_char @ 'f'
            | col_char @ 'g' => {
                match &spot.chars().nth(1).unwrap() {
                    '1' => None,
                    row_char @ '8'
                    | row_char @ '7'
                    | row_char @ '6'
                    | row_char @ '5'
                    | row_char @ '4'
                    | row_char @ '3'
                    | row_char @ '2' => {
                        let mut row = row_char.to_digit(10).unwrap();
                        row -= 1;
                        let y = format!("{}{}", plus_one_col(*col_char).unwrap(), row);
                        Some(y.as_bytes().try_into().unwrap())
                    
                    }
                    _ => None,
                }
            }
            _ => None,
        }

    };

    let bottom_left_diag = {
        match &spot.chars().nth(0).unwrap() {
            col_char @ 'b'
            | col_char @ 'c'
            | col_char @ 'd'
            | col_char @ 'e'
            | col_char @ 'f'
            | col_char @ 'g'
            | col_char @ 'h' => {
                match &spot.chars().nth(1).unwrap() {
                    '1' => None,
                    row_char @ '8'
                    | row_char @ '7'
                    | row_char @ '6'
                    | row_char @ '5'
                    | row_char @ '4'
                    | row_char @ '3'
                    | row_char @ '2' => {
                        let mut row = row_char.to_digit(10).unwrap();
                        row -= 1;
                        let y = format!("{}{}", minus_one_col(*col_char).unwrap(), row);
                        Some(y.as_bytes().try_into().unwrap())
                    
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    };

    let top_left_diag = {
        match &spot.chars().nth(0).unwrap() {
            col_char @ 'b'
            | col_char @ 'c'
            | col_char @ 'd'
            | col_char @ 'e'
            | col_char @ 'f'
            | col_char @ 'g'
            | col_char @ 'h' => {
                match &spot.chars().nth(1).unwrap() {
                    '8' => None,
                    row_char @ '7'
                    | row_char @ '6'
                    | row_char @ '5'
                    | row_char @ '4'
                    | row_char @ '3'
                    | row_char @ '2'
                    | row_char @ '1' => {
                        let mut row = row_char.to_digit(10).unwrap();
                        row += 1;
                        let y = format!("{}{}", minus_one_col(*col_char).unwrap(), row);
                        Some(y.as_bytes().try_into().unwrap())
                    
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    };
    let top_right_diag = {
        match &spot.chars().nth(0).unwrap() {
            col_char @ 'a'
            | col_char @ 'b'
            | col_char @ 'c'
            | col_char @ 'd'
            | col_char @ 'e'
            | col_char @ 'f'
            | col_char @ 'g' => {
                match &spot.chars().nth(1).unwrap() {
                    '8' => None,
                    row_char @ '7'
                    | row_char @ '6'
                    | row_char @ '5'
                    | row_char @ '4'
                    | row_char @ '3'
                    | row_char @ '2'
                    | row_char @ '1' => {
                        let mut row = row_char.to_digit(10).unwrap();
                        row += 1;
                        let y = format!("{}{}", plus_one_col(*col_char).unwrap(), row);
                        Some(y.as_bytes().try_into().unwrap())
                    
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    };

    let bounds = Bounds {
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

pub fn index_to_spot(index: usize) -> String{
    let row = index / 8;
    let col = index % 8;
    let first_char_opt = match col {
            0 => Some('a'),
            1 => Some('b'),
            2 => Some('c'),
            3 => Some('d'),
            4 => Some('e'),
            5 => Some('f'),
            6 => Some('g'),
            7 => Some('h'),
            _ => None,
    };
    let second_char_opt = match row {
        0 => Some('8'),
        1 => Some('7'),
        2 => Some('6'),
        3 => Some('5'),
        4 => Some('4'),
        5 => Some('3'),
        6 => Some('2'),
        7 => Some('1'),
        _ => None,
    };
    if let Some(first_char) = first_char_opt {
        if let Some(second_char) = second_char_opt {
            let spot = format!("{}{}",first_char,second_char);
            return spot;
        }
    }
    "".to_string()
}

pub fn convert_col(spot: &str) -> Result<usize, chess_errors::ChessErrors> {
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
            _ => {
                let msg = format!("Invalid notation");
                Err(chess_errors::ChessErrors::InvalidNotation(msg))
            },
        },
        None => {
            let msg = format!("Invalid notation");
            Err(chess_errors::ChessErrors::InvalidNotation(msg))
        },
    }?;

    Ok(col)
}

pub fn convert_row(spot: &str) -> Result<usize, chess_errors::ChessErrors> {
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
            _ =>{
                let msg = format!("Invalid notation");
                Err(chess_errors::ChessErrors::InvalidNotation(msg))
            },
        },
        None => {
            let msg = format!("Invalid notation");
            Err(chess_errors::ChessErrors::InvalidNotation(msg))
        },
    }?;

    Ok(row)
}

pub fn notation_to_index(spot: &str) -> Result<usize, chess_errors::ChessErrors> {
    let col = convert_col(spot)?;
    let row = convert_row(spot)?;
    let index = (row * 8) + col;
    Ok(index)
}

pub fn convert_move_notation_to_indexes(
    from_spot: &str,
    to_spot: &str,
) -> Result<(usize, usize), chess_errors::ChessErrors> {
    let from_index = notation_to_index(from_spot)?;
    let to_index = notation_to_index(to_spot)?;

    Ok((from_index, to_index))
}

pub fn convert_move_notation_to_xy(
    from_spot: &str,
    to_spot: &str,
) -> Result<(Point, Point), chess_errors::ChessErrors> {
    let from_col:u8  = (convert_col(from_spot)?) as u8;
    let from_row:u8 = convert_row(from_spot)?as u8;
    let to_col:u8  = convert_col(to_spot)?as u8;
    let to_row:u8 = convert_row(to_spot)?as u8;
    let from_point = Point{x: from_col,y:from_row};
    let to_point = Point{x: to_col,y: to_row};
    Ok((from_point, to_point))
}
