use crate::utils::{response::Response, router::Router};

pub fn endpoints(router: &mut Router) {
    // You start here!
    router.get("/", |_| {
        let response = Response::new();
        response.text("Hello, World", 200) // Returns hello, world with a status code 200
    });
}
