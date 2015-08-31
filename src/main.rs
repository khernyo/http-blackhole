
extern crate hyper;

use std::io::{Read, Write};

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::net::Fresh;

fn log(request: Request, response: Response<Fresh>) {
    let (socket_addr, method, headers, request_uri, http_version, mut body) = request.deconstruct();
    println!("----------------------------------------");
    println!("Request from {:?}", socket_addr);
    println!("Http version: {:?}", http_version);
    println!("Method: {:?}", method);
    println!("Headers: {:?}", headers);
    println!("Uri: {:?}", request_uri);

    let mut body_s = String::new();
    body.read_to_string(&mut body_s).unwrap();
    println!("Body: {:?}", body_s);

    let mut res = response.start().unwrap();
    res.write_all(b"").unwrap();
    res.end().unwrap();
}

fn main() {
    let _ = Server::http("127.0.0.1:8080").unwrap().handle(log);
}
