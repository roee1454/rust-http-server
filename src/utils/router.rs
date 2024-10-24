use crate::utils::request::Request;
use crate::utils::response::Response;
use serde_json::json;
use std::collections::HashMap;
use tokio::io::AsyncWriteExt;

type Handler = Box<dyn Fn(&Request) -> Response + Send + Sync>;

pub struct Router {
    get_routes: HashMap<String, Handler>,
    post_routes: HashMap<String, Handler>,
}

#[allow(dead_code)]
impl Router {
    pub fn new() -> Self {
        Self {
            get_routes: HashMap::new(),
            post_routes: HashMap::new(),
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

    // Method to handle requests
    pub async fn handle_request(&self, stream: &mut tokio::net::TcpStream, request: &Request) {
        let response = match request.method.as_str() {
            "GET" => {
                if let Some(handler) = self.get_routes.get(&request.route) {
                    let response = handler(request);
                    handle_ok(&response)
                } else {
                    handle_not_found()
                }
            }
            "POST" => {
                if let Some(handler) = self.post_routes.get(&request.route) {
                    let response = handler(request);
                    handle_ok(&response)
                } else {
                    handle_not_found()
                }
            }
            _ => handle_method_not_allowed(),
        };

        // Write the response back to the stream
        stream
            .write_all(response.as_bytes())
            .await
            .expect("Failed to write response back to the client");
    }
}

// Functions for handling responses
fn handle_ok(response: &Response) -> String {
    format!(
        "HTTP/1.1 {} {}\r\nContent-type: {}\r\n\r\n{}",
        response.status,
        response.status_text,
        response.content_type,
        if response.content_type == "application/json" {
            serde_json::to_string(&response.body).unwrap()
        } else {
            response.body.as_str().unwrap().to_string()
        }
    )
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
