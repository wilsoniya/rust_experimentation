use std::io::TcpStream;

fn main() {
    let mut socket = TcpStream::connect("wayne.wallitt.com", 81).unwrap();
    let method = "GET";
    let path = "/~wilsoniya/";
    socket.write(format!("{} {} HTTP/1.0\n\n", method, path).as_bytes());
    let response = socket.read_to_end();
    let res = response.unwrap();
    let response_body = std::str::from_utf8(res.as_slice()).unwrap();
    println!("{}", response_body);
    println!("response length: {}", res.len());


    println!("Request Headers:");
    println!("===============");
    for line in response_body.lines() {
        if line.trim_chars(|c: char| c.is_whitespace()).len() == 0 {
            break;
        }
        println!("{}", line);
    }
}
