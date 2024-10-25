use crate::utils::{response::Response, router::Router};
use serde_json::json;

pub fn endpoints(router: &mut Router) {
    router.get("/", |_| {
        let response = Response::new();
        response.text("Hello, World", 200) // Returns hello, world
    });

    router.get("/json", |_| {
        let response = Response::new();
        response.json(json!("{ \"id\": 1 }"), 200) // Returns a json object
    });

    router.post("/new-user", |req| {
        let body = &req.body; // Getting a reference to the request body which is an hashmap
        let username = body.get("username").unwrap(); // Getting a property from the body's hashmap
        // Update in database logic here...

        let response = Response::new();
        response.text(
            &format!("User with username {} finally has been created!", username),
            200,
        ) // Returns a response to the user containing his username
    })
}
