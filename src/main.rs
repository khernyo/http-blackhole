extern crate futures;
extern crate hyper;

use std::str;

use futures::{Future, Stream};
use hyper::service::service_fn;
use hyper::Body;
use hyper::Request;
use hyper::Response;
use hyper::Server;

type BoxFut = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn blackhole(req: Request<Body>) -> BoxFut {
    let method = req.method();
    let request_uri = req.uri();
    let http_version = req.version();
    let headers = req.headers();

    println!("----------------------------------------");
    println!("Http version: {:?}", http_version);
    println!("Method: {:?}", method);
    println!("Headers: {:?}", headers);
    println!("Uri: {:?}", request_uri);

    let body = req.into_body();

    Box::new(body.concat2().and_then(move |body| {
        println!("Body: {:?}", str::from_utf8(&body));
        Ok(Response::new(Body::empty()))
    }))
}

fn main() {
    let addr = ([127, 0, 0, 1], 18888).into();
    let server = Server::bind(&addr)
        .serve(|| service_fn(blackhole))
        .map_err(|e| eprintln!("server error: {}", e));
    println!("Listening on {}", addr);
    hyper::rt::run(server);
}
