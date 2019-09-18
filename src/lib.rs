use std::net;

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

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(get_index));
    cfg.route("/again", web::get().to(get_again));
    cfg.route("/again", web::post().to(post_again));
}

pub fn run_server<A: net::ToSocketAddrs>(addr: A) -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(config_app))
        .bind(addr)?
        .run()
}
