use rust_express::init::App;
use rust_express::utils::{response::Response, router::Router};

#[tokio::main]
async fn main() {
    let mut app = App::new();

    app.endpoints(move |router: &mut Router| {
        router.get("/", |_| {
            Response::new().text("Hello, World", 200)
        });
    });

    app.run(4000).await;
}
