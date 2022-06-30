mod chess_notation_utilities;
mod visual;

use visual::GameState;

fn main() {
    let mut game_state = GameState::default();
    println!("{}",game_state);

    if let Ok((from, to)) =  chess_notation_utilities::convert_move_notation_to_indexes("a8", "b8"){
        game_state.move_piece(from, to);
        println!("{}",game_state);
    }
    // let spot = "b2";
    // let bounds = chess_notation_utilities::get_bounds(spot, visual::PLAYER::BLACK); 
    // println!("{} bounds is\n{}", spot, bounds);
    
    let spot= "b2";
    if let Ok(index) = chess_notation_utilities::notation_to_index(spot) {
        if let Some(piece) = game_state.get_piece_at(index){
            println!("piece {}", piece.get_unicode_val());
            if  piece.move_forward_one(spot, &game_state).is_ok()  {
                println!("good");
            }else {
                println!("oops");
            }
            
        }
    }
    else {

    }
    
    
    
    // game_state.move_piece("a8", "b8");
    
    // println!("{}",game_state);
   
    
    

}
