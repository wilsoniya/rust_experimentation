mod echoer;

fn main() {
    echoer::listen("0.0.0.0", 1212).unwrap();
}
