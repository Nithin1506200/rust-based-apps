use tokio::io::AsyncReadExt;
use tokio::{io::AsyncWriteExt, net::TcpListener};

use std::collections::HashMap;
use std::net::TcpStream;
use std::{default, io};

enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    OPTION,
    DELETE,
}
impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "PATCH" => Method::PATCH,
            "OPTION" => Method::OPTION,
            "DELETE" => Method::DELETE,
            _ => Method::DELETE,
        }
    }
}
enum Version {
    HTTP1_1,
}
impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Version::HTTP1_1,
            _ => Version::HTTP1_1,
        }
    }
}
// #[derive(Debug)]
struct Request {
    method: Method,
    uri: String,
    version: Version,
    headers: HashMap<String, String>,
    query_params: HashMap<String, String>,
    path_params: HashMap<String, String>,
    reader: TcpStream,
}
type RequestParser = Result<Request, Error>;

enum Error {
    ParsingError,
    Utf8Error(std::string::FromUtf8Error),
    IOError(std::io::Error),
}
impl From<std::io::Error> for Error {
    fn from(internal_err: std::io::Error) -> Self {
        Error::IOError(internal_err)
    }
}
impl From<std::string::FromUtf8Error> for Error {
    fn from(internal_err: std::string::FromUtf8Error) -> Self {
        Error::Utf8Error(internal_err)
    }
}
impl Request {
    pub async fn new(mut reader: tokio::net::TcpStream) -> RequestParser {
        let mut buffer: Vec<u8> = Vec::new();

        let mut headers: HashMap<String, String> = HashMap::new();
        let mut first_line = String::new();
        // reader.read_u8(buf);
        loop {
            let mut b = reader.read_u8().await?;
            buffer.push(b);

            if b as char == '\n' {
                if first_line.is_empty() {
                    first_line = String::from_utf8(buffer.clone())?;
                } else {
                    if buffer.len() == 2 && buffer[0] as char == '\r' {
                        break;
                    }
                    let header_line = String::from_utf8(buffer.clone())?;
                    let mut iter = header_line.split(":");
                    let key = match iter.next() {
                        Some(k) => k,
                        None => return Err(Error::ParsingError),
                    };
                    let value = match iter.next() {
                        Some(k) => k,
                        None => return Err(Error::ParsingError),
                    };
                    headers.insert(key.to_string(), value.to_string());
                }
            }
        }
        let mut first_line_iter = first_line.split(" ");
        // let method = first_line_iter.next().unwrap().into();
        let uri_iter_next_unwrap = first_line_iter.next().unwrap().to_string();
        let mut uri_iter = uri_iter_next_unwrap.split("?");
        let uri = match uri_iter.next() {
            Some(s) => s,
            None => return Err(Error::ParsingError),
        };
        let mut queary_parms: HashMap<String, String> = HashMap::new();
        match uri_iter.next() {
            Some(s) => {
                for kv in s.split("&") {
                    let mut iter = kv.split("=");
                    let key = match iter.next() {
                        Some(k) => k,
                        None => return Err(Error::ParsingError),
                    };
                    let value = match iter.next() {
                        Some(k) => k,
                        None => return Err(Error::ParsingError),
                    };
                    queary_parms.insert(key.to_string(), value.to_string());
                }
            }
            None => (),
        }
        Ok(Request {
            method: todo!(),
            uri: uri.into(),
            version: first_line_iter.next().unwrap().into(),
            headers,
            query_params: queary_parms,
            path_params: HashMap::new(),
            reader: todo!(),
        })
    }
}

async fn process_socket(mut socket: tokio::net::TcpStream) -> io::Result<()> {
    socket.write_all(b"hellow workld").await?;
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listner = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (socket, _) = listner.accept().await?;
        process_socket(socket).await;
    }
}
