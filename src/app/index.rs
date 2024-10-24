use crate::utils::router::Router;
use crate::utils::response::Response;
use serde_json::json;

pub fn endpoints(router: &mut Router) {
    // Example #1
    router.get("/route/:id", |request| {
        let params = &request.params;
        let id = params.get("id").expect("Error while fetching id");
        Response::text(json!(format!("Hello, {id}")), 200)
    });
}
