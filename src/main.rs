mod init;
mod utils;
mod app;

#[tokio::main]
async fn main() {
    init::server::run(3000).await;
}
