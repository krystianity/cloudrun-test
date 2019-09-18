use std::process::{Child, Command};

pub fn setup() -> Child {
    Command::new("cargo")
        .args(&["run"])
        .env("PORT", "8088")
        .spawn()
        .expect("failed to start cloudrun_server")
}

pub fn tear_down(child: &mut Child) {
    child.kill().unwrap();
    child.wait().unwrap();
}
