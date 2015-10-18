
/// A Struct that stores some test data
#[derive(Debug)]
pub struct TestStruct {
    pub name: String,
    pub age: u8,
    pub favorite_words: Vec<String>,
}


pub fn moved_param(param: TestStruct) {
    println!("{:?}", param);
}

pub fn moved_and_returned_param(param: TestStruct) -> TestStruct {
    println!("{:?}", param);
    param
}

pub fn borrowed_param(param: &TestStruct) {
    println!("{:?}", param);
    do_something_with_string(&param.name);
}

fn do_something_with_string(str: &String) {
    println!("{}", str);
}

pub fn get_struct() -> TestStruct {
    TestStruct {
        name: "FART".to_string(),
        age: 31,
        favorite_words: vec!(
            "fart".to_string(),
            "turd".to_string(),
            "butt".to_string()
        )
    }
}



#[test]
fn test_moved_param() {
    let param1 = TestStruct{
        name: "FART".to_string(),
        age: 31,
        favorite_words: vec!(
            "fart".to_string(),
            "turd".to_string(),
            "butt".to_string())};

    moved_param(param1);
    // this would cause a compile error
//  moved_param(param1);
}
