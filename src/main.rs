mod chess_errors;
mod chess_notation_utilities;
mod visual;

use std::io::{stdin,stdout,Write};

use visual::GameState;

struct Game {
    state: GameState,
    player_turn: visual::PLAYER,
    turn_history: Vec<String>,
}
impl Default for Game {
    fn default() -> Self {
        Game {
            state:GameState::default(),
            player_turn: visual::PLAYER::WHITE,
            turn_history: Vec::new(),
        }
    }
   
}
impl Game {
    fn move_piece (&mut self ,chess_move: &str)->Result<(), chess_errors::ChessErrors> {
        let mut the_move = chess_move.to_lowercase();
        let index_of_dash = the_move.find("-");
        if let Some(index_of_dash) = the_move.find("-") {
            let from_spot = &the_move[0..index_of_dash];
            let to_spot = &the_move[index_of_dash+1..];
            if let Ok((from, to)) = chess_notation_utilities::convert_move_notation_to_indexes(from_spot,to_spot) {
                //TODO test move for valid
                self.state.move_piece(from, to);
            }
        }else {
            let msg = format!("Invalid notation");
            return Err(chess_errors::ChessErrors::InvalidNotation(msg));
        }
        
        Ok(())
    }
}
fn main() {
    let mut chess_game = Game::default();
    let mut game_over = false;
    while game_over == false {
        let mut move_notation=String::new();
        let prompt = {
            match  chess_game.player_turn{
                visual::PLAYER::WHITE => format!("White's turn:(e.g a2-b2,or quit)").to_string(),
                visual::PLAYER::BLACK => format!("Blacks's turn:(e.g a7-a6 or quit)").to_string(),
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
        chess_game.move_piece(&move_notation);
        chess_game.player_turn = match chess_game.player_turn {
            visual::PLAYER::WHITE => visual::PLAYER::BLACK,
            visual::PLAYER::BLACK => visual::PLAYER::WHITE,
        };
        
    
    }
    

    // let mut game_state = GameState::default();
    // println!("{}", game_state);

    // if let Ok((from, to)) = chess_notation_utilities::convert_move_notation_to_indexes("b2", "b3") {
    //     game_state.move_piece(from, to);
    //     println!("{}", game_state);
    // }
    // // let spot = "b2";
    // // let bounds = chess_notation_utilities::get_bounds(spot, visual::PLAYER::BLACK);
    // // println!("{} bounds is\n{}", spot, bounds);

    // let spot = "B2".to_string().to_lowercase();
    // if let Ok(index) = chess_notation_utilities::notation_to_index(&spot) {
    //     if let Some(piece) = game_state.get_piece_at(index) {
    //         println!("piece {}", piece.get_unicode_val());
    //         if piece.move_forward_one(&spot, &game_state).is_ok() {
    //             println!("good");
    //         } else {
    //             println!("oops");
    //         }
    //     }
    // } else {
    // }

   
}