pub fn convert_col ( spot: &str, ) ->Result<u8,&'static str>{
    let col = match  spot.chars().nth(0) {
        Some(first_char) => {
            match first_char {
                'a' => Ok(0),
                'b' => Ok(1),
                'c' => Ok(2),
                'd' => Ok(3),
                'e' => Ok(4),
                'f' => Ok(5),
                'g' => Ok(6),
                'h' => Ok(7),
                _ => Err("not valid move notation")
            }
        }, 
        None => {
            Err("not valid move notation")
        }
    }?;

    Ok(col)
}

pub fn convert_row ( spot: &str, ) ->Result<u8,&'static str>{
    let row = match  spot.chars().nth(1) {
        Some(first_char) => {
            match first_char {
                '8' => Ok(0),
                '7' => Ok(1),
                '6' => Ok(2),
                '5' => Ok(3),
                '4' => Ok(4),
                '3' => Ok(5),
                '2' => Ok(6),
                '1' => Ok(7),
                _ => Err("not valid move notation")
            }
        }, 
        None => {
            Err("not valid move notation")
        }
    }?;

    Ok(row)
}

pub fn convert_move_notation_to_indexes( from_spot: &str, to_spot: &str) ->Result<(u8,u8),&'static str>{
    let from_col = convert_col(from_spot)?;
    let to_col = convert_col(to_spot)?;
    let from_row = convert_row(from_spot)?;
    let to_row = convert_row(to_spot)?;
    println!("from col: {:?} from row {:?} to col {:?} to row {:?}", from_col,from_row,to_col,to_row);
    let from_index = (from_row * 8) + from_col;
    let to_index = (to_row * 8) + to_col;
    
    Ok((from_index, to_index))
}