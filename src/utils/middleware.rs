use crate::utils::request::Request;
use futures::future::BoxFuture;
use std::collections::HashMap;

pub type MiddlewareData = HashMap<String, String>;

type Middleware = Box<dyn Fn(&Request, &mut MiddlewareData) -> () + Send + Sync>;
type AsyncMiddleware = Box<
    dyn for<'a> Fn(&'a Request, &'a mut MiddlewareData) -> BoxFuture<'a, ()> + Send + Sync,
>;

pub enum EitherMiddleware {
    Sync(Middleware),
    Async(AsyncMiddleware),
}
