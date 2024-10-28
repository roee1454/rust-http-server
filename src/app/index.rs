use std::sync::Arc;
use super::start::Values;
use crate::utils::{response::Response, router::Router};

#[allow(unused_variables)]
pub async fn endpoints(router: &mut Router, values: Arc<Values>) {
    // You can define your routes here!
    router.get("/", |_| {
        let response = Response::new();
        response.text("Hello World!", 200)
    });

    // You can also define async routes!
    router.get_async("/async", |_| {
        async move {
            let response = Response::new();
            response.text("Hello World!", 200)
        }
    });
}
