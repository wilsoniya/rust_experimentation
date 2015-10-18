use std::rand::Rng;
use std::rand;
use std::fmt;

type IP = (u8, u8, u8, u8);

trait Printable {
    fn print(&self) -> ();
}

impl Printable for IP {
    fn print(&self) {
        let (o0, o1, o2, o3) = *self;
        println!("{}.{}.{}.{}", o0, o1, o2, o3);
    }
}

pub struct UUID {
    data: [u8, ..16],
}

impl fmt::Show for UUID {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for i in range(0, 16) {
            if i == 4 || i == 6 || i == 8 || i == 10 {
                fmt.write_str("-");
            }
            fmt.write_str(format!("{:x}", self.data[i]).as_slice());
        }
        Ok(())
    }
}

impl std::rand::Rand for UUID {
    fn rand<R: std::rand::Rng>(rng: &mut R) -> UUID {
        let mut data = [0u8, ..16];
        rng.fill_bytes(data.slice_mut(0, 16));
        UUID { data: data }
    }
}

fn main() {
    let ip: IP = rand::random();
    ip.print();
    let uuid: UUID = rand::random();
    println!("{}", uuid);
}
