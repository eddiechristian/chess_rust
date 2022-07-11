mod chess_errors;
mod chess_notation_utilities;
mod visual;

use std::io::{stdin,stdout,Write};

use visual::{GameState, PLAYER};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight
}

struct Game {
    state: GameState,
    turn_history: Vec<String>,
}
impl Default for Game {
    fn default() -> Self {
        Game {
            state:GameState::default(),
            turn_history: Vec::new(),
        }
    }
}
impl Game {
    fn game_from_turn_history(turn_history: &[&str]) -> Self {
        let mut chess_game = Game {
            state:GameState::default(),
            turn_history: Vec::new(),
        };
        for turn in turn_history {
            println!("turn {}",turn);
            chess_game.turn_history.push(turn.to_string());
            if let Err(e) =chess_game.move_piece(&turn, chess_game.state.player_turn){
                println!("{}",e);
            }else {
                chess_game.state.player_turn = match chess_game.state.player_turn {
                    visual::PLAYER::WHITE => visual::PLAYER::BLACK,
                    visual::PLAYER::BLACK => visual::PLAYER::WHITE,
                };
            }
        }
        chess_game
    }
    fn check_pieces_between(&self, from_spot: &str, to_spot: &str, dir: Direction)-> Result<(), chess_errors::ChessErrors>{
        let mut pos:String = to_spot.to_string();
        loop{
            println!("pos: {}", pos);
            if let Ok(bounds) = chess_notation_utilities::get_bounds(&pos){
                let next_pos_opt=  match dir{
                    Direction::Up => {
                            bounds.bottom
                    },
                    Direction::Down => {
                            bounds.top
                    },
                    Direction::Left => {
                            bounds.right
                    },
                    Direction::Right => {
                            bounds.left
                    },
                    Direction::DownLeft => {
                            bounds.top_right_diag
                    },
                    Direction::UpLeft => {
                            bounds.bottom_right_diag
                    },
                    Direction::UpRight => {
                            bounds.bottom_left_diag
                    },
                    Direction::DownRight => {
                            bounds.top_left_diag
                    },
                };
                println!("dir {:?}",dir);
                if let Some(next_pos_array) = next_pos_opt {
                    let x = std::str::from_utf8(&next_pos_array).unwrap();
                    pos= x.to_string();
                    if pos == from_spot {
                        break;
                    }
                    if let Ok(index) = chess_notation_utilities::notation_to_index(&pos) {
                        if let Some(piece) = self.state.get_piece_at(index) {
                            if piece.get_player() == self.state.player_turn{
                                return Err(chess_errors::ChessErrors::PieceBetween(pos));
                            }
                        }
                    }
                }else {
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
    fn is_move_valid(&self, from_spot: &str, to_spot: &str, whos_turn: visual::PLAYER, promotion_opt: Option<&str>)->Result<(), chess_errors::ChessErrors> {
        // first determine if piece at from is correct player.
        if let Ok(index) = chess_notation_utilities::notation_to_index(&from_spot) {
            if let Some(piece) = self.state.get_piece_at(index) {
                if piece.get_player() != whos_turn{
                    let msg = format!("{}",from_spot);
                    return Err(chess_errors::ChessErrors::WrongPlayer(msg));
                }
            } else {
                let msg = format!("{}",from_spot);
                return Err(chess_errors::ChessErrors::NoPiece(msg));
            }
        }
        //if too spot is current player its invalid
        if let Ok(index) = chess_notation_utilities::notation_to_index(&to_spot) {
            if let Some(piece) = self.state.get_piece_at(index) {
                if piece.get_player() == whos_turn{
                    let msg = format!("{}",to_spot);
                    return Err(chess_errors::ChessErrors::PlayerPieceAlreadyThere(msg));
                }
            } 
        }
        if  promotion_opt.is_some() {
            //promotions are only valid from 8th rank for pawn
            let from_row = chess_notation_utilities::convert_row(from_spot)?;
            let to_row = chess_notation_utilities::convert_row(to_spot)?;
            println!("from_row {:?} to_row {:?}",from_row,to_row);
            if whos_turn == PLAYER::WHITE && to_row !=  0 &&  from_row != 1 {
                let msg = format!("xxx{}",from_spot);
                return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
            }
            if whos_turn == PLAYER::BLACK && to_row != 8 && from_row!= 7 {
                let msg = format!("yyy{}",from_spot);
                return Err(chess_errors::ChessErrors::InvalidPromotion(msg));
            }
        }
        // the x and y deltas will tell what kind of move it is
        
        let (from_point, to_point) = chess_notation_utilities::convert_move_notation_to_xy(from_spot,to_spot)?;
        let delta_x: i8 = (from_point.x as i8 - to_point.x as i8) as i8;
        let delta_y: i8 = (from_point.y as i8 - to_point.y as i8) as i8;
        if delta_x == 0  {
            //vertical
            let dir = {
                if delta_y < 0 {
                    Direction::Down
                }else  {
                    Direction::Up
                }
            }; 
            if delta_y.abs() !=1 {
                self.check_pieces_between(from_spot, to_spot, dir)?;
            }
            if let Ok(index) = chess_notation_utilities::notation_to_index(&from_spot) {
                if let Some(piece) = self.state.get_piece_at(index) {
                    piece.move_vertical(to_spot, &self.state, delta_y)?;
                }
            }
        } else if delta_y == 0{
            //Horiz
            let dir = {
                if delta_x < 0 {
                    Direction::Right
                }else  {
                    Direction::Left
                }
            }; 
            if delta_x.abs() !=1 {
                self.check_pieces_between(from_spot, to_spot, dir)?;
            }
            if let Ok(index) = chess_notation_utilities::notation_to_index(&from_spot) {
                if let Some(piece) = self.state.get_piece_at(index) {
                    piece.move_horizontal(to_spot, &self.state, delta_x)?;
                }
            }
        }else if delta_x.abs() == delta_y.abs(){
            //diagonal
            //determine dir
            let dir = {
                if delta_x > 0 && delta_y > 0  {
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
                    piece.move_diagonal(to_spot, &self.state, delta_y )?;
                }
            }
            // if diagonal deltas must be equal, except for Knight

        }else if (delta_x.abs() == 2 && delta_y.abs() ==1) || (delta_x.abs() == 1 && delta_y.abs() ==2){
            if let Ok(index) = chess_notation_utilities::notation_to_index(&from_spot) {
                if let Some(piece) = self.state.get_piece_at(index) {
                    piece.move_knight(to_spot, &self.state)?;
                }
            }
        }

        //check for current player in check
        Ok(())
    }

    fn move_piece (&mut self ,chess_move: &str, whos_turn: visual::PLAYER)->Result<(), chess_errors::ChessErrors> {
        let the_move = chess_move.to_lowercase();
        if let Some(index_of_dash) = the_move.find("-") {
            let from_spot = &the_move[0..index_of_dash];
            let to_spot = &the_move[index_of_dash+1..index_of_dash+3];
            if let Ok((from, to)) = chess_notation_utilities::convert_move_notation_to_indexes(from_spot,to_spot) {
                let promotion = if let Some(index_of_p) = the_move.find("p") {
                    Some(&the_move[index_of_p+1 ..])
                } else {
                    None
                };
                self.is_move_valid(from_spot, to_spot, whos_turn, promotion)?;
                self.state.move_piece(from, to, promotion);
            } else {
                let msg = format!("Invalid notation");
                return Err(chess_errors::ChessErrors::InvalidNotation(msg));
            }
        }else {
            let msg = format!("Invalid notation");
            return Err(chess_errors::ChessErrors::InvalidNotation(msg));
        }
        
        Ok(())
    }
}
fn main() {
    let mut chess_game = Game::game_from_turn_history(&["a2-a4","b7-b5","a4-b5","f7-f5","b5-b6","b8-c6","b6-b7","f5-f4"]);
    let mut game_over = false;
    while game_over == false {
        let mut move_notation=String::new();
        let prompt = {
            match  chess_game.state.player_turn{
                visual::PLAYER::WHITE => format!("White's turn:(e.g a2-b2,a7-a8pr or quit)").to_string(),
                visual::PLAYER::BLACK => format!("Blacks's turn:(e.g a7-a6,a2-a1pq or quit)").to_string(),
            }
        };
        println!("{}", chess_game.state);
        println!("{}", prompt);
        let _=stdout().flush();
        stdin().read_line(&mut move_notation).expect("Did not enter a correct move");
        if let Some('\n')=move_notation.chars().next_back() {
            move_notation.pop();
        }
        if let Some('\r')=move_notation.chars().next_back() {
            move_notation.pop();
        }
        if move_notation == "quit" {
            break
        }
        if let Err(e) =chess_game.move_piece(&move_notation, chess_game.state.player_turn){
            println!("{}",e);
        }else {
            chess_game.turn_history.push(move_notation.to_string());
            chess_game.state.player_turn = match chess_game.state.player_turn {
                visual::PLAYER::WHITE => visual::PLAYER::BLACK,
                visual::PLAYER::BLACK => visual::PLAYER::WHITE,
            };
        }
    }
    

   
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_promotion() {
        let mut chess_game = Game::game_from_turn_history(&["a2-a4","b7-b5","a4-b5","f7-f5","b5-b6","b8-c6","b6-b7","f5-f4"]);
        assert_eq!(2,2);
    }

}