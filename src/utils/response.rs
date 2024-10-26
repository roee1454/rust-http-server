use std::{collections::HashMap, io::ErrorKind};

use crate::utils::request::Request;
use crate::utils::router::Router;
use base64::prelude::*;
use mime_guess::from_path;
use serde_json::{json, Value};
use std::fs;


#[allow(dead_code)]
pub struct Response {
    pub content_type: String,
    pub status_text: String,
    pub status: i64,
    pub body: Value,
    pub raw: String,
    pub cookies: HashMap<String, String>,
    pub is_file: bool,
}

#[allow(dead_code)]
impl Response {
    pub fn new() -> Self {
        Self {
            content_type: "text/plain".to_owned(),
            status_text: "OK".to_owned(),
            status: 200,
            body: json!(format!("")),
            raw: format!("HTTP/1.1 200 OK\r\nContent-type: text/plain\r\n\r\n"),
            cookies: HashMap::new(),
            is_file: false,
        }
    }

    pub fn text(&self, body: &str, status: i64) -> Response {
        Response {
            content_type: "text/plain".to_owned(),
            status_text: "OK".to_owned(),
            status,
            body: json!(body),
            raw: format!(
                "HTTP/1.1 {}\r\nContent-Type: text/plain\r\n\r\n{}",
                status,
                body
            ),
            cookies: self.cookies.to_owned(),
            is_file: false,
        }
    }

    pub fn json(&self, body: Value, status: i64) -> Response {
        Response {
            content_type: "application/json".to_owned(),
            status_text: "OK".to_owned(),
            status,
            body: body.to_owned(),
            raw: format!(
                "HTTP/1.1 {}\r\nContent-Type: application/json\r\n\r\n{}",
                status,
                serde_json::to_string(&body.to_owned()).unwrap()
            ),
            cookies: self.cookies.to_owned(),
            is_file: false,
        }
    }

    pub fn error(&self, body: &str, status: i64) -> Response {
        Response {
            status,
            body: json!(body),
            status_text: "Internal Server Error".to_owned(),
            content_type: "text/plain".to_owned(),
            raw: format!(
                "HTTP/1.1 {} Internal Server Error\r\nContent-Type: text/plain\r\n\r\n{}",
                status,
                body
            ),
            cookies: self.cookies.to_owned(),
            is_file: false,
        }
    }


    pub fn send_file(&self, path: &str, status: i64) -> Response {
        let filename = path.split("/").last().unwrap();
        let file_type = from_path(path).first_or_octet_stream().to_string();
        let mut file_content = String::new();

        match fs::read_to_string(path) {
            Ok(content) => file_content = content,
            Err(err) => {
                if err.kind() == ErrorKind::InvalidData {
                    let file_bytes = fs::read(path).expect("Error while reading file bytes");
                    file_content = BASE64_STANDARD.encode(file_bytes)
                } else {
                    eprint!("Error while reading file! {}", err.to_string())
                }
            }
        }

        Response {
            content_type: file_type.to_owned(),
            status_text: "OK".to_owned(),
            status,
            body: json!(format!("{filename}:{file_content}")),
            raw: format!(
                "HTTP/1.1 {}\r\nContent-Type: {}\r\n\r\n{}",
                status, file_type, self.body
            ),
            cookies: self.cookies.to_owned(),
            is_file: true,
        }
    }
}

pub async fn handle_response(
    stream: &mut tokio::net::TcpStream,
    request: &Request,
    router: &Router,
) {
    router.handle_request(stream, request).await;
}
