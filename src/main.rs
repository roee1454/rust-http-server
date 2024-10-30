use rust_http::init::App;
use rust_http::utils::{response::Response, router::Router};

#[tokio::main]
async fn main() {
    let mut app = App::new();

    let response_text = "Hello, World";

    app.endpoints(move |router: &mut Router| {
        // Configure routes
        router.get("/", |_| Response::new().text(response_text, 200));
    });

    app.run(4000).await;
}
