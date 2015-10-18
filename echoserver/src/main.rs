use std::io::{TcpListener, TcpStream};
use std::io::{Listener, Acceptor};
use std::io::BufferedReader;
use std::io::BufferedStream;

static BIND_ADDRESS: &'static str = "127.0.0.1:1234";

fn handle_conn(mut stream: TcpStream) {
    println!("Client connected");
    println!("Socket name: {}", stream.socket_name());
    println!("Peer name: {}", stream.peer_name());
    let mut bufstream = BufferedStream::new(stream);
    loop {
        match bufstream.read_line() {
            Ok(line) => {
                print!("{}", line);
                bufstream.write_str(format!("ECHO: {}", line).as_slice());
                bufstream.flush();
            },
            Err(_) => {
                println!("EOF detected. Dropping stream.");
                break
            },
        }
    }
    drop(bufstream);
}


fn main() {
    let listener = TcpListener::bind(BIND_ADDRESS);
    let mut acceptor = listener.listen();

    for stream in acceptor.incoming() {
        match stream {
            Ok(stream) => spawn(proc() { handle_conn(stream) }),
            Err(e) => println!("{}", e),
        }
    } 
}
