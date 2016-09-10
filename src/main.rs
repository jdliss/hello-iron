extern crate iron;
extern crate rustc_serialize;
extern crate router;

use iron::prelude::*;
use iron::status;
use rustc_serialize::json;
use router::Router;
use std::io::Read;

#[derive(RustcEncodable, RustcDecodable)]
struct Greeting {
    msg: String
}

fn main() {
    let mut router = Router::new();

    router.get("/", hello_world);
    router.post("/set", set_greeting);

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let greeting = Greeting { msg: "Hello, World!".to_string() };
        let payload = json::encode(&greeting).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    fn set_greeting(request: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        let request: Greeting = json::decode(&payload).unwrap();
        let greeting = Greeting { msg: request.msg };
        let payload = json::encode(&greeting).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    println!("Server running on localhost:3000");
    Iron::new(router).http("localhost:3000").unwrap();
}
