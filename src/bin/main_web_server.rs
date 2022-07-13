
use actix_cors::Cors;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/chess")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and mongoDB")
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // let cors = Cors::default()
        //       .allowed_origin("*")
        //       .allowed_methods(vec!["GET", "POST"])
        //       .max_age(3600);

        App::new()
            .wrap(Cors::permissive() )
            .service(hello)
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await

}