#[macro_use]
extern crate actix_web;

use std::net;

use actix_web::{web, App, HttpResponse, HttpServer, Result};

#[get("/")]
fn get_index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("GET - Hello world!"))
}

#[get("/again")]
fn get_again() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("GET - Hello world again!"))
}

#[post("/again")]
fn post_again() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("POST - Hello world again!"))
}

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(get_index);
    cfg.service(get_again);
    cfg.service(post_again);
}

pub fn run_server<A: net::ToSocketAddrs>(addr: A) -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(config_app))
        .bind(addr)?
        .run()
}
