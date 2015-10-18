use std::os;

mod input_parser;

fn parse_args() -> String {
    let args = os::args();
    match args.len() {
        2 => args[1].clone(),
        _ => fail!("Incorrect args!"),
    }
}


fn main() {
    let fpath = parse_args(); 
    let parsed = input_parser::DecayScenario::from_file(fpath.as_slice());
    let sim = parsed.simulate();
    println!("{}", sim);
}
