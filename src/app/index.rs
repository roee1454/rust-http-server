use crate::utils::router::Router;
use crate::utils::response::Response;
use serde_json::json;

pub fn endpoints(router: &mut Router) {
    // Example #1
    router.get("/users", |_| {
        Response::text(json!("Hello, World!"), 200)
    });
}
