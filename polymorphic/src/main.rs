
trait Poly {
    fn get() -> Self;
}

#[deriving(Show)]
struct Bar {
    data: String
}

#[deriving(Show)]
struct Foo {
    data: int
}

impl Poly for Bar {
    fn get() -> Bar {
        Bar { data: String::from_str("Fart") }
    }
}

impl Poly for Foo {
    fn get() -> Foo {
        Foo { data: 5 }
    }
}

fn get_new_poly<T: Poly>() -> T {
//  let x: T = Poly::get();
//  x
    Poly::get()
}

fn main() {
    let a: Foo = get_new_poly();
    println!("{}", a);
}
