extern crate hyper;

use std::str;

use hyper::service::{service_fn, make_service_fn};
use hyper::{Body, Request, Response, Server};

async fn blackhole(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
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
    let body_bytes = hyper::body::to_bytes(body).await?;
    println!("Body: {:?}", str::from_utf8(&body_bytes));

    Ok(Response::new(Body::empty()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    let addr = ([127, 0, 0, 1], 18888).into();

    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(blackhole))});
    let server = Server::bind(&addr).serve(service);
    println!("Listening on {}", addr);
    server.await?;
    Ok(())
}
