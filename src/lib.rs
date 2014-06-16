#![crate_id = "hello"]
#![deny(missing_doc)]

//! A simple ingot to serve "Hello, world!" to all requests.

extern crate iron;

#[allow(unused_imports)] // This is here for the sake of example, and should be removed
use iron::{Iron, Ingot, Furnace, Alloy, Request, Response, ServerT};
use iron::ingot::{Status, Continue};

/// The hello `Ingot`.
#[deriving(Clone)]
pub struct HelloWorld;

impl<Rq: Request, Rs: Response> Ingot<Rq, Rs> for HelloWorld {
    fn enter(&mut self, _request: &mut Rq, response: &mut Rs, _alloy: &mut Alloy) -> Status {
        let _ = response.write(bytes!("Hello World!"));
        Continue
    }
    fn exit(&mut self, _request: &mut Rq, _response: &mut Rs, _alloy: &mut Alloy) -> Status {
        println!("Served \"Hello, World\".");
        Continue
    }
}
