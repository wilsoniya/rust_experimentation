extern crate iron;
extern crate router;
extern crate postgres;

use iron::prelude::*;
use iron::status;
use iron::headers::{Headers, ContentType};
use iron::modifiers::Header;

use router::Router;

//fn doit(req: &mut Request) -> IronResult<Response> {
fn doit(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(("Hello World!", status::Ok)))
}

//fn poop<F>(func: F) where F : Fn(&str) -> &str {
fn poop<F: Fn(&str) -> &str>(func: F) {
	println!("{}", func("butt"));
}

fn index(req: &mut Request) -> IronResult<Response> {
    let mut headers = Headers::new();
    headers.set(ContentType::html());

    Ok(Response::with(("
                        <html>
                            <head>
                                <title>The rain in spain</title>
                            </head>
                            <body>
                                <h1>Falls mainly</h1>
                                <hr>
                                <p>In the plain.</p>
                                <hr>
                                &copy; 2016 Michael Wilson
                            </body>
                        </html>
                       ", status::Ok, Header(ContentType::html()))))
}

fn account(req: &mut Request) -> IronResult<Response> {
    let mut headers = Headers::new();
    headers.set(ContentType::html());

    let account_name: &str = req.extensions.get::<Router>().unwrap().find("name").unwrap();
    println!("account name: {}", account_name);

    Ok(Response::with((format!("
                        <html>
                            <head>
                                <title>Account</title>
                            </head>
                            <body>
                                Who are you?
                                Oh yeah, you're {}
                            </body>
                        </html>
                       ", account_name), status::Ok, Header(ContentType::html()))))
}

fn main() {
    let mut router = Router::new();
    router.get("/", index);
    router.get("/account/:name", account);

    let iron = Iron::new(router);
    iron.http("localhost:3000");

}
