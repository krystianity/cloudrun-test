use std::collections::HashMap;
use reqwest;

mod common;

#[test]
fn test_server_index() {

    let port = "8089";
    let mut cloudrun_server = common::setup(port);
    let endpoint = format!("http://localhost:{}/", port);

    let client = reqwest::Client::new();
    client
        .get(endpoint.as_str())
        .send()
        .map_err(|_| ())
        .and_then(|mut response| {
            assert_eq!(response.status(), 200);
            let body = response.text().unwrap();
            println!("{}", body);
            assert_eq!(body, "Hi.");
            Ok(())
        }).expect("Request failed.");

    common::tear_down(&mut cloudrun_server);
}

#[test]
fn test_server_index_param() {

    let port = "8090";
    let mut cloudrun_server = common::setup(port);
    let endpoint = format!("http://localhost:{}/test", port);

    let client = reqwest::Client::new();
    client
        .get(endpoint.as_str())
        .send()
        .map_err(|_| ())
        .and_then(|mut response| {
            assert_eq!(response.status(), 200);
            let body = response.text().unwrap();
            println!("{}", body);
            assert_eq!(body, "Hello test.");
            Ok(())
        }).expect("Request failed.");

    common::tear_down(&mut cloudrun_server);
}

#[test]
fn test_server_get_json() {

    let port = "8091";
    let mut cloudrun_server = common::setup(port);
    let endpoint = format!("http://localhost:{}/json/test", port);

    let client = reqwest::Client::new();
    client
        .get(endpoint.as_str())
        .send()
        .map_err(|_| ())
        .and_then(|mut response| {
            assert_eq!(response.status(), 200);
            let body: HashMap<String, String> = response.json().unwrap();
            println!("{:#?}", body);
            assert_eq!(body.get("name").unwrap(), "test");
            Ok(())
        }).expect("Request failed.");

    common::tear_down(&mut cloudrun_server);
}

#[test]
fn test_server_post_json() {

    let port = "8092";
    let mut cloudrun_server = common::setup(port);
    let endpoint = format!("http://localhost:{}/json", port);

    let client = reqwest::Client::new();
    client
        .post(endpoint.as_str())
        .header("content-type", "application/json")
        .body("{\"name\": \"test\"}")
        .send()
        .map_err(|_| ())
        .and_then(|mut response| {
            assert_eq!(response.status(), 200);
            let body: HashMap<String, String> = response.json().unwrap();
            println!("{:#?}", body);
            assert_eq!(body.get("name").unwrap(), "test");
            Ok(())
        }).expect("Request failed.");

    common::tear_down(&mut cloudrun_server);
}
