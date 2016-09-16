extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use iron::response::{WriteBody,ResponseBody};
use std::io::Write;

use router::Router;

#[derive(Clone)]
struct Livewire {
    name: String,
}

impl WriteBody for Livewire {
    fn write_body(&mut self, res: &mut ResponseBody) -> Result<(),std::io::Error> {
        loop {
            try!(res.write(self.name.as_bytes()));
        }
    }
}

fn handler(_req: &mut Request) -> IronResult<Response> {
    let mut r = Response::new();
    r.status = Some(status::Ok);
    let l = Livewire{name:"joao".into()};
    let ml = Box::new(l);
    r.body = Some(ml);
    Ok(r)
}

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "index");

    println!("running on localhost:3000");
    Iron::new(router).http("localhost:3000").unwrap();
}
