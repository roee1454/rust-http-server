use crate::utils::request::Request;
use crate::utils::router::Router;
use serde_json::{json, Value};

#[allow(dead_code)]
pub struct Response {
    pub content_type: String,
    pub status_text: String,
    pub status: i64,
    pub body: Value,
    pub raw: String,
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
        }
    }

    pub fn text(body: Value, status: i64) -> Response {
        Response {
            content_type: "text/plain".to_owned(),
            status_text: "OK".to_owned(),
            status,
            body: body.to_owned(),
            raw: format!(
                "HTTP/1.1 {}\r\nContent-Type: text/plain\r\n\r\n{}",
                status,
                body.as_str().unwrap()
            ),
        }
    }

    pub fn json(body: Value, status: i64) -> Response {
        Response {
            content_type: "application/json".to_owned(),
            status_text: "OK".to_owned(),
            status,
            body: body.to_owned(),
            raw: format!(
                "HTTP/1.1 {}\r\nContent-Type: application/json\r\n\r\n{}",
                status,
                serde_json::to_string(&body).unwrap()
            ),
        }
    }

    pub fn error(body: Value, status: i64) -> Response {
        Response {
            status: status.to_owned(),
            body: body.to_owned(),
            status_text: "Internal Server Error".to_owned(),
            content_type: "text/plain".to_owned(),
            raw: format!(
                "HTTP/1.1 500\r\nContent-Type: text/plain\r\n\r\n{}",
                body.as_str().unwrap()
            ),
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
