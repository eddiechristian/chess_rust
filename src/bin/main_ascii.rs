use chess::game::{Game, GameStatus};
use chess::visual::{GameState, WebGame, PLAYER};
use std::io::{stdin, stdout, Write};
use std::{env, io};

fn main() {
    //https://hub.qovery.com/guides/tutorial/create-a-blazingly-fast-api-in-rust-part-1/
    //https://github.com/evoxmusic/twitter-clone-rust/blob/master/Cargo.toml
    //https://www.petercollingridge.co.uk/tutorials/svg/interactive/dragging/

    let mut chess_game = Game::game_from_turn_history(&["d2-d3", "e7-e6", "b2-b3", "d8-h4"]);
    //let mut chess_game = Game::default();
    let d: WebGame = (&chess_game.state).into();

    println!("webState: {:?}", d);

    chess_game.get_validated_moves(chess_game.state.player_turn);
    let mut game_over = false;
    while game_over == false {
        let mut move_notation = String::new();
        let prompt = {
            match chess_game.state.player_turn {
                PLAYER::WHITE => "White's turn (e.g. e2-e4, Ng1-f3, e4xd5, e7-e8=Q, O-O, or quit)".to_string(),
                PLAYER::BLACK => "Black's turn (e.g. e7-e5, Ng8-f6, d5xe4, e2-e1=N, O-O-O, or quit)".to_string(),
            }
        };
        println!("{}", chess_game.state);
        println!("{}", prompt);
        let _ = stdout().flush();
        stdin()
            .read_line(&mut move_notation)
            .expect("Did not enter a correct move");
        if let Some('\n') = move_notation.chars().next_back() {
            move_notation.pop();
        }
        if let Some('\r') = move_notation.chars().next_back() {
            move_notation.pop();
        }
        if move_notation == "quit" {
            break;
        }
        if let Err(e) = chess_game.move_piece(&move_notation, chess_game.state.player_turn) {
            println!("{}", e);
        } else {
            chess_game.turn_history.push(move_notation.to_string());
            chess_game.state.player_turn = match chess_game.state.player_turn {
                PLAYER::WHITE => PLAYER::BLACK,
                PLAYER::BLACK => PLAYER::WHITE,
            };
            match chess_game.game_status() {
                GameStatus::Check => println!("Check!"),
                GameStatus::Checkmate(winner) => {
                    println!("{}", chess_game.state);
                    println!("Checkmate! {:?} wins", winner);
                    game_over = true;
                }
                GameStatus::Stalemate => {
                    println!("{}", chess_game.state);
                    println!("Stalemate! The game is a draw");
                    game_over = true;
                }
                GameStatus::FiftyMoveDraw => {
                    println!("{}", chess_game.state);
                    println!("Draw by the 50-move rule");
                    game_over = true;
                }
                GameStatus::InProgress => {}
            }
        }
    }
}
