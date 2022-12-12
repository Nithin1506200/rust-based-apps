use std::collections::HashMap;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
pub type RequestParser = Result<Request, Error>;
#[derive(Debug)]
pub enum Method {
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
#[derive(Debug)]
pub enum Version {
    HTTP1_1,
}
impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HTTP1_1 => f.write_str("HTTP/1.1"),
        }
    }
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
pub struct Request {
    pub method: Method,
    pub uri: String,
    pub version: Version,
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub path_params: HashMap<String, String>,
}
pub struct Response<'a> {
    pub status: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: &'a str,
}
pub struct StatusCode {
    pub code: usize,
    pub msg: &'static str,
}
impl StatusCode {
    pub fn ok() -> Self {
        StatusCode {
            code: 200,
            msg: "OK",
        }
    }
}
pub struct Connection {
    pub request: Request,

    pub socket: tokio::net::TcpStream,
}
impl Connection {
    pub async fn new(mut socket: tokio::net::TcpStream) -> Result<Connection, Error> {
        let request = Request::new(&mut socket).await?;
        // let response = Response {};
        Ok(Connection { request, socket })
    }
    pub async fn respond<'a>(&mut self, resp: Response<'a>) -> Result<(), Error> {
        // self.socket.write_all(body)
        self.socket
            .write_all(
                format!(
                    "{:?} {:?} {:?}",
                    self.request.version, resp.status.code, resp.status.msg
                )
                .as_bytes(),
            )
            .await?;
        Ok(())
    }
}

pub enum Error {
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
    pub async fn new(reader: &mut tokio::net::TcpStream) -> RequestParser {
        let mut buffer: Vec<u8> = Vec::new();

        let mut headers: HashMap<String, String> = HashMap::new();
        let mut first_line = String::new();
        // reader.read_u8(buf);
        loop {
            let b = reader.read_u8().await?;
            buffer.push(b);
            println!("loop : {:?}", b as char);
            if b as char == '\n' {
                if first_line.is_empty() {
                    first_line = String::from_utf8(buffer[0..buffer.len() - 2].to_vec())?;
                    println!("first line{:?}", first_line);
                    buffer.clear();
                } else {
                    if buffer.len() == 2 && buffer[0] as char == '\r' {
                        break;
                    }
                    let header_line = String::from_utf8(buffer[0..buffer.len() - 2].to_vec())?;
                    buffer.clear();
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
        let method: Method = first_line_iter.next().unwrap().into();
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
            method,
            uri: uri.into(),
            version: first_line_iter.next().unwrap().into(),
            headers,
            query_params: queary_parms,
            path_params: HashMap::new(),
        })
    }
}
