#![allow(dead_code)]

use client::*;

fn crash(client: &Client) {
    let size = 1024 * 1024 * 1024 * 4;
    let resp = client.crash(CrashRequest { size });
    if let Ok(resp) = resp {
        println!("{:?} MiB", resp.payload.len() / (1024 * 1024));
    } else {
        println!("{:?}", resp);
    };
    return;
}

fn stream(client: &Client) {
    let size = 1024 * 1024 * 1024 * 5;
    client.stream(CrashRequest { size }).unwrap();
}

fn main() {
    env_logger::init();

    let client = Client::new("localhost", 50056).unwrap();
    // crash(&client);
    stream(&client)
}
