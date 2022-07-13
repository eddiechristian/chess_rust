

#[macro_use]
extern crate actix_web;

use std::io;

use actix_web::web::{ Json};
use actix_web::{middleware, App, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize};

pub const APPLICATION_JSON: &str = "application/json";

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidMovesRequest {
    pub message: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ValidMovesResponse {
    pub valid_moves: Vec<String>,
}
/// create a tweet `/tweets`
#[post("/valid_moves")]
pub async fn valid_moves(valid_move_req: Json<ValidMovesRequest>) -> HttpResponse {
    let mut moves= Vec::new();
    moves.push("a2-a4".to_string());
    moves.push("b2-b4".to_string());
    let response = ValidMovesResponse {
        valid_moves: moves,
    };
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(response)
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    
    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(valid_moves)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}