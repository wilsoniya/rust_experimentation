extern crate "rustc-serialize" as rustc_serialize; 

use rustc_serialize::json;

#[derive(Debug, RustcDecodable)]
pub enum ChatEvent {
    Variant(i32)
}

fn main() {
    let serialized = "{\"variant\": \"Variant\", \"fields\": []}";

    match json::decode(serialized) {
        Ok(event) => {
            let event: ChatEvent = event;
            println!("Ok {:?}", event);
        }, 
        Err(e) => println!("Err {:?}", e)
    }
}
