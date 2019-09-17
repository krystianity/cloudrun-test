use std::thread;
use std::sync::mpsc;
use log::info;
use actix_web::{server, web, App, HttpResponse, HttpServer, Responder};

fn get_index() -> impl Responder {
    HttpResponse::Ok().body("GET - Hello world!")
}

fn get_again() -> impl Responder {
    HttpResponse::Ok().body("GET - Hello world again!")
}

fn post_again() -> impl Responder {
    HttpResponse::Ok().body("POST - Hello world again!")
}

pub struct CloudrunServer {
    server_handle: thread::JoinHandle<()>,
    addr: server::Server
}

impl CloudrunServer {

    pub fn new(address: String) -> CloudrunServer {

        let (tx, rx) = mpsc::channel();
        let server_handle = thread::spawn(move || {

            let sys = actix_rt::System::new("cloudrun-server");
            let addr = HttpServer::new(|| {
                App::new()
                    .route("/", web::get().to(get_index))
                    .route("/again", web::get().to(get_again))
                    .route("/again", web::post().to(post_again))
            })
            .bind(address)
            .unwrap()
            .shutdown_timeout(60)
            .start();

            let _ = tx.send(addr);
            let _ = sys.run();
        });

        let addr = rx.recv().unwrap();
        CloudrunServer { server_handle, addr }
    }

    #[allow(dead_code)]
    pub fn print_id(&self) -> () {
        info!(
            "Server running in thread {:?}",
            self.server_handle.thread().id()
        );
        ()
    }

    pub fn join(self) -> thread::Result<()> {
        self.server_handle.join()
    }
}