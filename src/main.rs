#![allow(dead_code, unused_variables, unused_imports)]

use std::net::TcpListener;

use newslatter::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind a random port.");
    run(listener)?.await
}
