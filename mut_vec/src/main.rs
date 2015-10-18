#[deriving(Show)]
struct Structure {
    v: Vec<i64>, 
}
fn main() {
    let mut s = Structure {v: vec![0]};
    s.v.get_mut(0) += 1;
    println!("{}", s);

}
