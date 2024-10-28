use crate::app::index::endpoints;
use crate::app::start::Values;
use crate::utils::request::parse_request_data;
use crate::utils::response::handle_response;
use crate::utils::router::Router;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;

pub async fn run(port: usize, values: Values) {
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind ip address to tcp listener");
    println!("Server is running at: http://localhost:{}", port);
    let values = Arc::new(values);
    loop {
        let (mut stream, _) = listener
            .accept()
            .await
            .expect("Failed to accept the client's established connection");
        let values = Arc::clone(&values);
        tokio::spawn(async move {
            handle_stream(&mut stream, values).await;
        });
    }
}

async fn handle_stream(stream: &mut tokio::net::TcpStream, values: Arc<Values>) {
    let request = parse_request_data(stream).await;
    let mut router = Router::new();
    endpoints(&mut router, values).await;
    handle_response(stream, &request, &router).await;
    stream.flush().await.expect("Failed to flush stream");
}
