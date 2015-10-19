extern crate levenshtein;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (str1, str2) = match args.len() {
        l if l >= 3 => {
            (&args[1], &args[2])
        },
        _ => panic!("Error: At least two command-line parameters must be provided.")
    };

    let chars1: Vec<char> = str1.chars().collect();
    let chars2: Vec<char> = str2.chars().collect();
    let dist = levenshtein::distance(&chars1[..], &chars2[..]);

    println!("The distance between \"{}\" and \"{}\" is {}.",
             str1, str2, dist);
}
