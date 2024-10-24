use crate::utils::router::Router;
use crate::utils::response::Response;
use serde_json::json;

pub fn endpoints(router: &mut Router) {
    // Example #1
    router.get("/", |_| {
        Response::text(json!(format!("Hello world")), 200) // returns a response object as a string with a status code of 200
    });
}
