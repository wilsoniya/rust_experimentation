// use std::result::Result;
// use std::net::UdpSocket;
// use std::str::from_utf8;
// use std::error::Error;
// use std::convert::From;
// use std::io::Error as IoError;
// use std::str::Utf8Error;
//
// pub enum EchoError {
//     Io(IoError),
//     Utf(Utf8Error),
// }
// impl From<IoError> for EchoError {
//     fn from(err: IoError) -> EchoError {
//         EchoError::Io(err)
//     }
// }
// impl From<Utf8Error> for EchoError {
//     fn from(err: Utf8Error) -> EchoError {
//         EchoError::Utf(err)
//     }
// }
//
//
// pub fn listen(bind_addr: &str, port: u16) -> Result<(), EchoError> {
//     let socket =  try!(UdpSocket::bind((bind_addr, port)));
//     let mut buf = [0u8; 1024];
//
//     println!("reading from socket {}:{}...", bind_addr, port);
//
//     loop {
//         let (amt, src) = try!(socket.recv_from(&mut buf));
//         let response: &str = try!(from_utf8(&buf));
//         println!("Received {} byte(s) from {}: '{}'", amt, src, &response[0..amt]);
//     }
//
//     Ok(())
// }

use std::result::Result;
use std::net::UdpSocket;
use std::str::from_utf8;
use std::error::Error;

pub fn listen(bind_addr: &str, port: u16) -> Result<(), Box<Error + Send + Sync>> {
    let socket =  try!(UdpSocket::bind((bind_addr, port)));
    let mut buf = [0u8; 1024];

    println!("reading from socket {}:{}...", bind_addr, port);

    loop {
        let (amt, src) = try!(socket.recv_from(&mut buf));
        let response: &str = try!(from_utf8(&buf));
        println!("Received {} byte(s) from {}: '{}'", amt, src, &response[0..amt]);
    }
}
