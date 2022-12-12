use tokio::io::AsyncReadExt;
use tokio::{io::AsyncWriteExt, net::TcpListener};

use std::collections::HashMap;
use std::{default, io};

mod http;
use http::*;

async fn handle(mut socket: tokio::net::TcpStream) -> Result<(), Error> {
    let mut connection = Connection::new(socket).await?;
    println!(
        "method:{:?} \nuri:{:?}\nversion:{:?}\nheaders:{:?}",
        connection.request.method,
        connection.request.uri,
        connection.request.version,
        connection.request.headers
    );
    connection
        .respond(Response {
            status: StatusCode::ok(),
            headers: HashMap::new(),
            body: &String::new(),
        })
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listner = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (socket, _) = listner.accept().await?;
        handle(socket).await;
    }
}
