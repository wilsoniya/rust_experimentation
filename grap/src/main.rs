extern crate regex;

use std::fs::File;
use std::io::{Read, Result};
use std::process::exit;
use regex::Regex;


static FNAME: &'static str = "/home/wilsoniya/devel/rust_experimentation/grap/pg730.txt";

fn main() {
    let contents = match get_file(FNAME) {
        Ok(contents) => {
            contents
        },
        Err(_) => {
            panic!("Could not open {}", FNAME)
        }
    };

    println!("file size: {}", contents.len());

    let needle = "fagin";
    let occs = get_occurrences(needle, &contents);
    println!("occs: {:?}", occs);

    let lines = get_lines(&contents);
    println!("lines: {:?}", &lines[0..20]);

}

fn get_file(fname: &str) -> Result<String> {
    let mut contents = String::new();
    let mut fptr = try!(File::open(fname));
    fptr.read_to_string(&mut contents);
    Ok(contents)
}

fn get_occurrences(needle: &str, haystack: &str) -> Vec<usize> {
    let pat = format!("(?i){}", needle);
    let re = Regex::new(&pat).unwrap();

    re.find_iter(haystack).map(|(start, _)| start).collect()
}

fn get_lines<'a> (contents: &'a str) -> Vec<&'a str> {
    let re = Regex::new("\r*\n+\r*").unwrap();
    re.split(contents).collect()
}
