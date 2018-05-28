extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::str;

use futures::{Future, Stream};
use hyper::server::Http;
use hyper::server::Request;
use hyper::server::Response;
use hyper::server::Service;

struct BlackHole;

impl Service for BlackHole {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let (method, request_uri, http_version, headers, body) = req.deconstruct();
        println!("----------------------------------------");
        println!("Http version: {:?}", http_version);
        println!("Method: {:?}", method);
        println!("Headers: {:?}", headers);
        println!("Uri: {:?}", request_uri);

        Box::new(body.concat2().and_then(move |body| {
            println!("Body: {:?}", str::from_utf8(&body));
            Ok(Response::new())
        }))
    }
}

fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();
    println!("Listening on {}", addr);
    let server = Http::new().bind(&addr, || Ok(BlackHole)).unwrap();
    server.run().unwrap();
}
