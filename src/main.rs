use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use log::info;
use std::env;

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

    // Cloud Run requires us to listen to a dynamic port
    let port_env_key = "PORT";
    let mut http_port = String::new();
    match env::var(port_env_key) {
        Ok(val) => http_port = val,
        Err(e) => println!("Couldn't interpret {}: {}", port_env_key, e),
    }
    let http_bind_str = &format!("0.0.0.0:{}", http_port);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/again", web::get().to(get_again))
            .route("/again", web::post().to(post_again))
    })
    .bind(http_bind_str)
    .unwrap()
    .run()
    .unwrap();
}
