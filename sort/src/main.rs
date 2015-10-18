extern crate core;

use core::clone::Clone;
use std::cmp::PartialOrd;
use std::cmp::max;

fn print_vec<T: std::fmt::Show>(v: &Vec<T>) {
    print!("[");
    for i in v.iter() {
        print!("{},", i)
    }
    println!("]");
}

fn sort<T: PartialOrd + Clone>(list: &Vec<T>) -> Vec<T> {
    return if list.len() == 1 {
        list.clone()
    } else {
        let halfway = list.len() / 2;
        let first = sort(&list.slice(0, halfway).to_vec());
        let last = sort(&list.slice(halfway, list.len()).to_vec());
        let mut ret: Vec<T> = Vec::new();
        for i in range(0, max(first.len(), last.len())) {
            if i >= first.len() {
                ret.push(last[i].clone());
            } else if i >= last.len() {
                ret.push(first[i].clone());
            } else {
                let (a, b) = match first[i].partial_cmp(&last[i]).unwrap() {
                    Less => (first[i].clone(), last[i].clone()),
                    Greater => (last[i].clone(), first[i].clone()),
                    Equal => (first[i].clone(), last[i].clone()),
                };
                ret.push(a);
                ret.push(b);
            }
        }
        ret
    }
}

fn main() {
    let nums = vec![1i, 5i, 3i, 2i, 10i];
    print_vec(&nums);
    print_vec(&sort(&nums));
}
