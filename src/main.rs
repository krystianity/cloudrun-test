use std::env;
use log::info;
use cloudrun_server::CloudrunServer;

fn main() {

    env_logger::init();
    info!("Starting server..");

    // Cloud Run requires us to listen to a dynamic port
    let port_env_key = "PORT";
    let mut http_port = String::new();
    match env::var(port_env_key) {
        Ok(val) => http_port = val,
        Err(e) => println!("Couldn't interpret {}: {}", port_env_key, e),
    }
    let http_bind_str = format!("0.0.0.0:{}", http_port);

    let server = CloudrunServer::new(http_bind_str);
    server.join().unwrap();
}
