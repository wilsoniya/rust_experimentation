extern crate serialize;
use serialize::json;
use serialize::json::Json;

#[deriving(Decodable, Encodable)]
pub struct MyObject {
    name: String,
    age: i64,
    favorite_color: String, 
    favorite_numbers: Vec<i64>,
    more_json: Json,
}

fn main() {
    let obj = MyObject {
        name: "Michael Wilson".to_string(),
        age: 30,
        favorite_color: "Green".to_string(),
        favorite_numbers: vec![1, 1, 2, 3, 5, 8, 13],
        more_json: json::Null,
    };

    let str = "{
        name: \"michael wilson\",
        list: [1,2,3,4],
        something_else: null 
    }";

    let j = Json::from_str(str);

//  let encoded = json::encode(&obj);
//  println!("{}", encoded);
}
