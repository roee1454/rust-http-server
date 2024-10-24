mod init;
mod utils;
mod index;

#[tokio::main]
async fn main() {
    init::server::run(3000).await;
}
