#![crate_id = "hello"]
#![deny(missing_doc)]
#![feature(phase)]

//! A simple ingot to serve "Hello, world!" to all requests.

extern crate iron;
extern crate http;
#[phase(plugin, link)]
extern crate log;

use iron::{Ingot, Alloy, Request, Response};
use iron::ingot::{Status, Continue};

use http::status;

/// The hello `Ingot`.
#[deriving(Clone)]
pub struct HelloWorld;

impl HelloWorld {
    /// Create a new instance of the hello `Ingot`.
    pub fn new() -> HelloWorld {
        HelloWorld
    }
}

impl<Rq: Request, Rs: Response> Ingot<Rq, Rs> for HelloWorld {
    /// Serve "Hello, world!"
    ///
    /// In the case of an error, return a status of 500: InternalServerError
    fn enter(&mut self, _req: &mut Rq, res: &mut Rs, _alloy: &mut Alloy) -> Status {
        match res.write(bytes!("Hello, world!")) {
            Ok(()) => (),
            Err(_) => {
                let status = res.status_mut();
                *status = status::InternalServerError;
            }
        }
        Continue
    }

    /// Debug what you did.
    ///
    /// Prints `Served "Hello, World".` if RUST_LOG is set to 4.
    /// This function does not need to be implemented (neither does `enter`).
    /// `Ingot` implements a default function which does nothing.
    /// It is implemented for the sake of example.
    fn exit(&mut self, _req: &mut Rq, _res: &mut Rs, _alloy: &mut Alloy) -> Status {
        debug!("Served \"Hello, World\".");
        Continue
    }
}
