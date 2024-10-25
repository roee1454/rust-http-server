use tokio::io::AsyncWriteExt;
use crate::app::index::endpoints;
use crate::utils::request::parse_request_data;
use crate::utils::response::handle_response;
use crate::utils::router::Router;


pub async fn run(port: usize) {
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind ip address to tcp listener");
    println!("Server is running at: http:://localhost:{}", port);
    loop {
        let (mut stream, _) = listener.accept().await.expect("Failed to accept the client's established connection");
        tokio::spawn(async move {
            handle_stream(&mut stream).await;
        });
    }
}

async fn handle_stream(stream: &mut tokio::net::TcpStream,) {
    let request = parse_request_data(stream).await;
    let mut router = Router::new();
    endpoints(&mut router);
    handle_response(stream, &request, &router).await;
    stream.flush().await.expect("Failed to flush stream");
}
