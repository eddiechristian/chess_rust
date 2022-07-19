use std::io::{stdin,stdout,Write};
use std::{env, io};
use chess::visual::{GameState, PLAYER,WebGame};
use chess::game::{Game};

fn main() {
    //https://hub.qovery.com/guides/tutorial/create-a-blazingly-fast-api-in-rust-part-1/
    //https://github.com/evoxmusic/twitter-clone-rust/blob/master/Cargo.toml
//https://www.petercollingridge.co.uk/tutorials/svg/interactive/dragging/

    //  let mut chess_game = Game::game_from_turn_history(&["a2-a4","b7-b5","a4-b5","f7-f5","b5-b6","b8-c6",
    //      "b6-b7","f5-f4","a1-a7","g7-g6","d2-d4","h7-h5","d4-d5","h5-h4", "b2-b4","c6-a5", "b4-b5","c7-c5"]);  
    let mut chess_game = Game::default();   
    let d:WebGame=  (&chess_game.state).into();
    
    println!("webState: {:?}", d);

    chess_game.get_validated_moves(chess_game.state.player_turn);
    let mut game_over = false;
    while game_over == false {
        let mut move_notation=String::new();
        let prompt = {
            match  chess_game.state.player_turn{
                PLAYER::WHITE => format!("White's turn:(e.g a2-b2,a7-a8pr or quit)").to_string(),
                PLAYER::BLACK => format!("Blacks's turn:(e.g a7-a6,a2-a1pq or quit)").to_string(),
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
                PLAYER::WHITE => PLAYER::BLACK,
                PLAYER::BLACK => PLAYER::WHITE,
            };
        }
    }
}