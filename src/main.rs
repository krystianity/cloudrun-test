use cloudrun_test::run_server;
use log::info;
use std::env;

// Cloud Run requires us to listen to a dynamic port
const PORT_ENV_KEY: &str = "PORT";

fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting server..");

    let http_port = env::var(PORT_ENV_KEY).expect("did not find PORT environment variable");
    let http_bind_str = format!("0.0.0.0:{}", http_port);
    run_server(http_bind_str)
}
