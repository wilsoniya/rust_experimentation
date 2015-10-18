mod funcs;

fn get_param() -> funcs::TestStruct {
    funcs::TestStruct {
        name: "FART".to_string(),
        age: 31,
        favorite_words: vec!(
            "fart".to_string(),
            "turd".to_string(),
            "butt".to_string()
        )
    }
}

fn main() {
    let param = get_param();
    funcs::moved_param(param);

    let param = get_param();
    let param = funcs::moved_and_returned_param(param);
    let param = funcs::moved_and_returned_param(param);
    let param = funcs::moved_and_returned_param(param);

    let param = get_param();
    funcs::borrowed_param(&param);
    funcs::borrowed_param(&param);

    let boxd_struct = Box::new(funcs::get_struct());
    funcs::borrowed_param(&*boxd_struct);
    funcs::borrowed_param(&*boxd_struct);
}
