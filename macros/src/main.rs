#![feature(macro_rules)]

#[deriving(Show, PartialEq)]
enum Fart<T> {
    Butt(T),
    Turd,
}

macro_rules! test {
    ($($fart:expr),+) => (
        {
            $(
                println!("{} = {}", stringify!($fart), $fart);
            )+
        } 
    )
}

fn main() {
    test!(1i + 1, 0i + 10, "a" == "b");

    let f1 = Butt(1i);
    let f2 = Butt("braap!");
    let f3: Fart<Option<u8>> = Turd;
    let f4 = Butt(12i);
    let f5 = Butt(12i);

    test!(f1, f2, f3);
    test!(f1 == f4, f4 == f5);
}
