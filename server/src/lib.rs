extern crate hyper;
extern crate futures;
extern crate serde;
extern crate serde_json;

extern crate github_webhook_data as ghwh;

use futures::future::{ok, Future};
use futures::Stream;

use hyper::server::{Http, Request, Response, Service};
use hyper::{Chunk, Method, StatusCode};

struct Listener;

impl Service for Listener {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        match (req.method(), req.path()) {
            (&Method::Post, "/") => Box::new(req.body().concat2().map(handle)),
            (&Method::Get, "/help") => Box::new(req.body().concat2().map(help)),
            _ => Box::new(ok(Response::new().with_status(StatusCode::NotFound))),
        }
    }
}

fn help(_chunk: Chunk) -> Response {
    Response::new().with_body(r#"{"endpoints":[
  {
    "path": "/",
    "method": "POST",
    "description": "POST GitHub webhook API data to this endpoint"
  },
  {
    "path": "/help",
    "method": "GET",
    "description": "Get a list of available endpoints and their accepted methods as JSON"
  }
]}"#)
}

fn handle(chunk: Chunk) -> Response {
    let bytes = chunk.into_iter().collect::<Vec<_>>();
    let json = match String::from_utf8(bytes) {
        Ok(json) => json,
        Err(_e) => return Response::new().with_status(StatusCode::BadRequest),
    };

    if let Ok(event) = ghwh::events::pull_request::Event::from_json(&json) {
        println!("{:?}", event);
    }

    Response::new()
}

pub fn run(addr: &str) {
    let addr = addr.parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Listener)).unwrap();
    server.run().unwrap();
}
