use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use crate::utils::{request::parse_request_data, response::handle_response, router::Router};

pub struct App {
    endpoints: Option<Arc<dyn Fn(&mut Router) + Send + Sync>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            endpoints: None,
        }
    }

    pub fn endpoints<F>(&mut self, endpoints: F)
    where
        F: Fn(&mut Router) + Send + Sync + 'static,
    {
        self.endpoints = Some(Arc::new(endpoints));
    }

    pub async fn run(self, port: i128) {
        let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{port}"))
            .await
            .expect("Error while binding connection to public address");
        println!("Server is running at: http://localhost:{}", port);

        let app_ref = Arc::new(self);
        loop {
            let (mut stream, _) = listener
                .accept()
                .await
                .expect("Failed to accept the client's established connection");
            let app = Arc::clone(&app_ref);
            tokio::spawn(async move {
                app.handle_stream(&mut stream).await;
            });
        }
    }

    async fn handle_stream(&self, stream: &mut tokio::net::TcpStream) {
        let request = parse_request_data(stream).await;
        let mut router = Router::new();

        // Configure routes via callback
        if let Some(ref endpoints) = self.endpoints {
            endpoints(&mut router);
        }

        handle_response(stream, &request, &router).await;
        stream.flush().await.expect("Failed to flush stream");
    }
}
