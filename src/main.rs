mod api;

use axum::Router;
use std::{env, net::SocketAddr, process::exit, str::FromStr};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut addr = "127.0.0.1:8080";

    if args.len() > 1 {
        addr = &args[1];
    }

    let app: Router = Router::new()
        // Lorem
        .merge(api::lorem::router());

    let sock_addr = match SocketAddr::from_str(addr) {
        Ok(addr) => addr,
        Err(_) => {
            println!("Invalid address: {addr}");
            exit(1)
        }
    };
    println!("{}", logo());
    println!("Listening on {sock_addr}");

    axum::Server::bind(&sock_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn logo() -> String {
    String::from_str(
        r"

,------.                                  ,---.    ,------.                     
|  .---' ,--,--. ,---.  ,---.      ,---. /  .-'    |  .---',---.  ,---.  ,---.  
|  `--, ' ,-.  |(  .-' | .-. :    | .-. ||  `-,    |  `--,| .-. || .-. |(  .-'  
|  `---.\ '-'  |.-'  `)\   --.    ' '-' '|  .-'    |  |`  ' '-' '' '-' '.-'  `) 
`------' `--`--'`----'  `----'     `---' `--'      `--'    `---'  `---' `----'  

",
    )
    .unwrap()
}
