use std::io::stdin;

// fn tokenize() {
//     let s: &str = "The,rain,in,spain,falls,mainly,on,the,plain";
//     for a in s.split_str(",") {
//         println!("{}", a);
//     }
// }
// 
// fn stdin_echo() {
//     let mut _stdin = stdin();
//     print!("Say something: ")
//     for line in _stdin.lines() {
//         match line {
//             Ok(s)  => print!("You said: {}", s),
//             Err(_) => println!("BLARG"),
//         }
//         print!("Say something: ")
//     }
//     println!("EOF received. Exiting.");
// }

#[deriving(Show)]
struct Data {
    tot_case: i64,
    tot_death: i64, 
    num_rows: i64,
}

fn sum_csv_field() {
    let mut _stdin = stdin(); 
    let mut data = Data{tot_case: 0, tot_death: 0, num_rows: 0};
    for (i, line) in _stdin.lines().enumerate() {
        data.num_rows += 1;
        if i == 0 {
            continue;
        }

        match line {
            Ok(s)  => {
                let fields: Vec<&str> = s.as_slice().split('\t').collect();

                match from_str::<i64>(fields[5]) {
                    Some(tot_case) => data.tot_case += tot_case,
                    None => ()
                }
                match from_str::<i64>(fields[8]) {
                    Some(tot_death) => data.tot_death += tot_death,
                    None => ()
                }
            },
            Err(_) => println!("BLARG"),
        }
    }
    println!("{}", data);
}

fn main() {
//  tokenize();
//  stdin_echo(); 
    sum_csv_field()
}
