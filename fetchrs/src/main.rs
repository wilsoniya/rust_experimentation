extern crate futures;
extern crate hyper;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod ipify;

use std::io::{self, Write};

use hyper::Client;
use hyper::rt::{self, Future, Stream};

fn main() {

	let uri = "http://httpbin.org/ip".parse().unwrap();

	println!("uri: {}", uri);

    let future;
    {
        let client = Client::new();
        future = client
            .get(uri)
            .and_then(|res| {
                println!("Response: {}", res.status());
                println!("Headers: {:#?}", res.headers());

                // The body is a stream, and for_each returns a new Future
                // when the stream is finished, and calls the closure on
                // each chunk of the body...
                res.into_body().for_each(|chunk| {
                    io::stdout().write_all(&chunk)
                        .map_err(|e| panic!("example expects stdout is open, error={}", e))
                })
            })

            //.map(|res| {
            //	println!("Response: {}", res.status());
              //    let body = res.into_body();
              //    body.for_each(|chunk| {
              //        io::stdout().write_all(&chunk);
              //    });
              //    println!("body: {:?}", body);
            //})
            .map(|_| {
                println!("\n\nDone.");
            })
            .map_err(|err| {
                println!("Error: {}", err);
            });
    }

    rt::run(future);
}
