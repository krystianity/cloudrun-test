use log::info;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

fn get_index() -> impl Responder {
    HttpResponse::Ok().body("GET - Hello world!")
}

fn get_again() -> impl Responder {
    HttpResponse::Ok().body("GET - Hello world again!")
}

fn post_again() -> impl Responder {
    HttpResponse::Ok().body("POST - Hello world again!")
}

fn main() {
    env_logger::init();
    info!("Starting..");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/again", web::get().to(get_again))
            .route("/again", web::post().to(post_again))
    })
    .bind("0.0.0.0:80")
    .unwrap()
    .run()
    .unwrap();
}
