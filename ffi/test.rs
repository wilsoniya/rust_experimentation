
#[link(name = "test")]
extern {
    fn test();
}

fn main() {
    unsafe { test() };
}
