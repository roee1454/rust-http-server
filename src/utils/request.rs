use crate::utils::helpers::{parse_body, parse_query_params};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::io::AsyncReadExt;

#[derive(Deserialize, Serialize, Clone)]
pub struct Request {
    pub method: String,
    pub route: String,
    pub host: String,
    pub user_agent: String,
    pub accept: String,
    pub connection: String,
    pub raw: String,
    pub content_length: usize,
    pub body: HashMap<String, String>,
    pub query: HashMap<String, String>,
}

impl Request {
    fn new() -> Self {
        Self {
            method: "GET".to_owned(),
            route: "/".to_owned(),
            host: "".to_owned(),
            user_agent: "".to_owned(),
            accept: "".to_owned(),
            connection: "keep-alive".to_owned(),
            content_length: 0,
            raw: "".to_owned(),
            body: HashMap::new(),
            query: HashMap::new(),
        }
    }
}

pub async fn parse_request_data(stream: &mut tokio::net::TcpStream) -> Request {
    stream
        .readable()
        .await
        .expect("stream failed to be readable");
    let mut buffer = [0u8; 4096];
    let n = stream
        .read(&mut buffer)
        .await
        .expect("Failed to read request buffer");
    let request_data = String::from_utf8_lossy(&buffer[..n]).to_string();
    let mut request = Request::new();
    request.raw = request_data.clone();
    let mut lines = request_data.split("\r\n");

    if let Some(first_line) = lines.next() {
        let mut parts = first_line.split_whitespace();
        request.method = parts.next().unwrap_or("GET").to_string();
        let full_route = parts.next().unwrap_or("/");
        let mut route_parts = full_route.splitn(2, '?');
        request.route = route_parts.next().unwrap_or("/").to_string();
        let query_string = route_parts.next().unwrap_or("");
        request.query = parse_query_params(query_string);
    }

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let mut header_parts = line.splitn(2, ": ");
        let header_key = header_parts.next().unwrap_or("").to_lowercase();
        let header_value = header_parts.next().unwrap_or("").to_string();

        match header_key.as_str() {
            "host" => request.host = header_value,
            "user-agent" => request.user_agent = header_value,
            "accept" => request.accept = header_value,
            "connection" => request.connection = header_value,
            "content-length" => request.content_length = header_value.parse().unwrap_or(0),
            _ => {}
        }
    }

    let body = lines.collect::<Vec<&str>>().join("\r\n");
    let parsed_body = parse_body(&body);
    request.body = parsed_body;
    request
}
