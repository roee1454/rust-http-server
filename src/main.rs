mod init;
mod utils;
mod app;

#[tokio::main]
async fn main() {
    let values = app::start::start().await;
    init::server::run(4000, values).await;
}
