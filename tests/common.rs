use std::process::{Child, Command};
use std::thread;
use std::time;

pub fn setup(port: &str) -> Child {

    let child = Command::new("cargo")
        .args(&["run"])
        .env("PORT", port)
        .spawn()
        .expect("failed to start cloudrun_server");

    thread::sleep(time::Duration::from_secs(1));
    child
}

pub fn tear_down(child: &mut Child) {
    child.kill().unwrap();
    child.wait().unwrap();
    thread::sleep(time::Duration::from_secs(1));
}
