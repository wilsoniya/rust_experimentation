// The Look and Say sequence is an interesting sequence of numbers where each term is given by
// describing the makeup of the previous term.
//
// The 1st term is given as 1. The 2nd term is 11 ('one one') because the first term (1) consisted
// of a single 1. The 3rd term is then 21 ('two one') because the second term consisted of two 1s.
// The first 6 terms are:
//
// 1
// 11
// 21
// 1211
// 111221
// 312211

use std::os::args;
use std::io::stdio::stderr;

fn gen_ls_sequence(seq: Vec<int>) -> Vec<int> {
    let mut ret: Vec<int> = vec![];
    let mut last_num = seq[0];
    let mut last_count = 0i;
    for num in seq.iter() {
        if last_num == *num {
            last_count += 1;
        } else {
            ret.push(last_count);
            ret.push(last_num);
            last_count = 1;
            last_num = *num; 
        }
    }

    ret.push(last_count);
    ret.push(last_num);

    ret 
}

fn looksay(num_iters: int) -> Vec<Vec<int>> {
    let mut last_seq = vec![1i];
    let mut ret: Vec<Vec<int>> = vec![last_seq.clone()];
    for i in range(0i, num_iters) {
        let cur_seq = gen_ls_sequence(last_seq);
        ret.push(cur_seq.clone());
        last_seq = cur_seq;
    }

    ret
}

fn get_args() -> Result<int, &'static str> {
    match args().as_slice().slice(1, args().len()) {
        [] => Err("num_iters not supplied"),
        [ref n] => {
            match from_str::<int>(n.as_slice()) {
                Some(num_iters) => Ok(num_iters),
                None => Err("num_iters has incorrect type; int expected.")
            }
        },
        [_, ..] => Err("too many parameters supplied"),
    }
}

fn print_looksay(res: Vec<Vec<int>>) {
    for seq in res.iter() {
        println!("{}", *seq);
    }
}

fn main() {
    match get_args() {
        Ok(num_iters) => {
            println!("{} iters will be taken.", num_iters);
            let res = looksay(num_iters);
            print_looksay(res);
        },
        Err(msg) => {
            std::os::set_exit_status(1);
            stderr().write_line(format!("Error: {}", msg).as_slice());
        }
    }

}
