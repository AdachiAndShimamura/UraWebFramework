use anyhow::Result;
use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::service::Service;
use hyper::{Request, Response};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

#[derive(Default)]
pub struct Router {
    router: HashMap<String, ServiceHandler>,
}

impl Router {
    pub fn add_handler(&mut self, path: String, service_handler: ServiceHandler) -> Result<()> {
        self.router.insert(path, service_handler).unwrap();
        Ok(())
    }

    pub fn new() -> Router {
        Router {
            router: HashMap::new(),
        }
    }
    pub fn new_test() -> Router {
        let mut router = Router {
            router: HashMap::new(),
        };
        router
            .router
            .insert("test".to_string(), ServiceHandler::new());
        router
    }
}

impl Service<Request<Incoming>> for Router {
    type Response = Response<Full<Bytes>>;
    type Error = anyhow::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        match self.router.get(req.uri().path()){
            Some(handler) => {
                let res = handler.handle(req);
                Box::pin(async move { Ok(res) })
            }
            None => {Box::pin(async { Ok(Response::new(Full::new(Bytes::from("test!")))) })}
        }

    }
}

pub struct ServiceHandler {
    handle_func: Box<dyn Fn(Request<Incoming>) -> Response<Full<Bytes>>>,
}

impl ServiceHandler {
    fn new() -> ServiceHandler {
        ServiceHandler {
            handle_func: Box::new(Self::test),
        }
    }
    pub fn test(request: Request<Incoming>) -> Response<Full<Bytes>> {
        Response::new(Full::new(Bytes::from("test data")))
    }
    pub fn handle(&self, request: Request<Incoming>) -> Response<Full<Bytes>> {
        (self.handle_func)(request)
    }
}

unsafe impl Send for ServiceHandler {}

unsafe impl Sync for ServiceHandler {}
