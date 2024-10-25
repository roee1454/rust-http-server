use crate::utils::request::Request;
use crate::utils::response::Response;
use serde_json::json;
use std::collections::HashMap;
use tokio::io::AsyncWriteExt;

type Handler = Box<dyn Fn(&Request) -> Response + Send + Sync>;

pub struct Router {
    get_routes: HashMap<String, Handler>,
    post_routes: HashMap<String, Handler>,
    put_routes: HashMap<String, Handler>,
    delete_routes: HashMap<String, Handler>,
}

#[allow(dead_code)]
impl Router {
    pub fn new() -> Self {
        Self {
            get_routes: HashMap::new(),
            post_routes: HashMap::new(),
            put_routes: HashMap::new(),
            delete_routes: HashMap::new(),
        }
    }

    pub fn get<F>(&mut self, route: &str, handler: F)
    where
        F: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        self.get_routes.insert(route.to_string(), Box::new(handler));
    }

    pub fn post<F>(&mut self, route: &str, handler: F)
    where
        F: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        self.post_routes
            .insert(route.to_string(), Box::new(handler));
    }

    pub fn put<F>(&mut self, route: &str, handler: F)
    where
        F: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        self.put_routes.insert(route.to_string(), Box::new(handler));
    }

    pub fn delete<F>(&mut self, route: &str, handler: F)
    where
        F: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        self.delete_routes
            .insert(route.to_string(), Box::new(handler));
    }

    pub async fn handle_request(&self, stream: &mut tokio::net::TcpStream, request: &Request) {
        let response = match request.method.as_str() {
            "GET" => {
                if let Some(handler) = self.get_routes.get(&request.route) {
                    let mut response = handler(request);
                    handle_ok(&mut response)
                } else {
                    handle_not_found()
                }
            }
            "POST" => {
                if let Some(handler) = self.post_routes.get(&request.route) {
                    let mut response = handler(request);
                    handle_ok(&mut response)
                } else {
                    handle_not_found()
                }
            }

            "PUT" => {
                if let Some(handler) = self.put_routes.get(&request.route) {
                    let mut response = handler(request);
                    handle_ok(&mut response)
                } else {
                    handle_not_found()
                }
            }

            "DELETE" => {
                if let Some(handler) = self.delete_routes.get(&request.route) {
                    let mut response = handler(request);
                    handle_ok(&mut response)
                } else {
                    handle_not_found()
                }
            }

            _ => handle_method_not_allowed(),
        };

        stream
            .write_all(response.as_bytes())
            .await
            .expect("Failed to write response back to the client");
    }
}

fn handle_ok(response: &mut Response) -> String {
    let mut cookies = String::new();
    if !response.cookies.is_empty() {
        for (k, v) in response.cookies.iter() {
            cookies += &format!(
                "Set-Cookie: {}={}; HttpOnly; Path=/; Domain=example.com; Max-Age=3600\r\n",
                k, v
            );
        }
    }

    let mut content_disposition = String::new();
    let body_content: String;
    let content_length;

    if response.is_file {
        let body_string = response.body.to_string();
        let mut parts = body_string.splitn(2, ":");

        let filename = parts.next().unwrap_or("file.txt").trim_matches('"');

        let file_data = parts
            .next()
            .unwrap_or("")
            .replace("\\r\\n", "\n")
            .replace("\\\"", "\"")
            .trim_end_matches("\"")
            .into();

        content_disposition = format!(
            "Content-Disposition: attachment; filename=\"{}\"\r\n",
            filename
        );
        body_content = file_data;
        content_length = body_content.len();
    } else {
        body_content = response.body.as_str().unwrap_or("").to_string();
        content_length = body_content.len();
    }

    let new_response = format!(
        "HTTP/1.1 {} {}\r\n{}Content-Type: {}\r\n{}Content-Length: {}\r\n\r\n{}",
        response.status,
        response.status_text,
        content_disposition,
        response.content_type,
        cookies,
        content_length,
        body_content
    );
    new_response
}

fn handle_not_found() -> String {
    format!(
        "HTTP/1.1 404 Not Found\r\nContent-type: application/json\r\n\r\n{}",
        serde_json::to_string(&json!({
            "error": "Page not found"
        }))
        .unwrap()
    )
}

fn handle_method_not_allowed() -> String {
    format!(
        "HTTP/1.1 405 Method Not Allowed\r\nContent-type: application/json\r\n\r\n{}",
        serde_json::to_string(&json!({
            "error": "This method is not allowed"
        }))
        .unwrap()
    )
}
