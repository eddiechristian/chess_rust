// #[macro_use]
// extern crate serde;

use std::collections::HashMap;

use actix_cors::Cors;
use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

use chess::game::Game;
use chess::visual::WebGame;
use serde::{Deserialize, Serialize};

#[get("/chess")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and mongoDB")
}


#[derive(Debug, Deserialize, Serialize)]
pub struct ValidMovesRequest {
    pub message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidMovesResponse {
    pub moves: HashMap<String,Vec<String>>,
    pub web_game: WebGame,
}

#[post("/valid_moves")]
async fn valid_moves(req: Json<ValidMovesRequest>) -> impl Responder {
    let mut chess_game = Game::game_from_turn_history(&["a2-a4"]); 
    let web_game:WebGame=  (&chess_game.state).into();
    
    //println!("webState: {:?}", d);
    
    let mut moves_map = HashMap::new();
    moves_map.insert("a2".to_string(), vec!["a3".to_string()]);
    moves_map.insert("b2".to_string(), vec![ "b3".to_string(), "b4".to_string()]);
    moves_map.entry("a2".to_string()).or_insert_with(|| {Vec::new()}).push("a4".to_string());
    
    let mut resp = ValidMovesResponse {
        moves: moves_map,
        web_game: web_game,
    };
    HttpResponse::Ok().json(resp)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive() )
            .service(hello)
            .service(valid_moves)
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await

}