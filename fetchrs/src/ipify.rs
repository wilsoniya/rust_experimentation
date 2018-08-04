use std::convert::From;
use std::str::FromStr;

use futures;
use hyper::{self, Client};
use hyper::rt::{self, Future, Stream};

use serde_json;

#[derive(PartialEq, Debug)]
pub struct IP(u8, u8, u8, u8);

impl FromStr for IP {
    type Err = IPParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut octets = Vec::new();
        for octet_str in s.split('.') {
            match u8::from_str(octet_str) {
                Ok(octet) => octets.push(octet),
                Err(_) => return Err(IPParseError::OctetOutOfRange),
            }
        }

        if octets.len() != 4 {
            return Err(IPParseError::WrongNumOctets);
        }

        Ok(IP(octets[0], octets[1], octets[2], octets[3]))
    }
}

#[derive(Debug, PartialEq)]
pub enum IPParseError {
    OctetOutOfRange,
    WrongNumOctets,
}

#[derive(Debug)]
pub enum IpifyError {
    RequestError,
    ParseError,
}

impl From<hyper::Error> for IpifyError {
    fn from(e: hyper::Error) -> IpifyError {
        IpifyError::RequestError
    }
}

impl From<IPParseError> for IpifyError {
    fn from(e: IPParseError) -> IpifyError {
        IpifyError::ParseError
    }
}

impl From<::std::str::Utf8Error> for IpifyError {
    fn from(e: ::std::str::Utf8Error) -> IpifyError {
        IpifyError::ParseError
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct IpifyResponse {
    origin: String,
}

impl IpifyResponse {
    fn from_str(json: &str) -> Self {
        serde_json::from_str(json).unwrap()
    }
}

pub fn get_ip() -> impl Future<Item=IP, Error=IpifyError> {
	let uri = "http://httpbin.org/ip".parse().unwrap();
    let client = Client::new();
    client
        .get(uri)
        .map_err(|e|{ e.into() })
        .and_then(|res| {
            res
                .into_body()
                .concat2()
                .map_err(|e|{ e.into() })
        })
        .and_then(|body_bytes| {
            ::std::str::from_utf8(&body_bytes)
                .map(|bytes| { bytes.to_owned() })
                .map_err(|e| { e.into() })
        })
        .and_then(|body| {
            let response = IpifyResponse::from_str(&body);
            IP::from_str(&response.origin)
                .map_err(|e|{ e.into() })
        })
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deserialize() {
        let actual = IpifyResponse::from_str("{\"origin\": \"fartturdbutt\"}");
        let expected = IpifyResponse{ origin: String::from("fartturdbutt") };
        assert!(actual == expected);
    }

    #[test]
    fn test_parse_ip() {
        let expected = Ok(IP(127, 0, 0, 1));
        let actual = IP::from_str("127.0.0.1");

        assert!(expected == actual);
    }

    #[test]
    fn test_fail_parse_ip() {
        let expected = Err(IPParseError::OctetOutOfRange);
        let actual = IP::from_str("256.0.0.1");
        assert!(expected == actual);

        let expected = Err(IPParseError::WrongNumOctets);
        let actual = IP::from_str("127.0.0.1.1");
        assert!(expected == actual);

        let expected = Err(IPParseError::OctetOutOfRange);
        let actual = IP::from_str("256.0.0.1.1");
        assert!(expected == actual);
    }

    #[test]
    fn test_ipify() {
        let f = futures::future::ok::<(), ()>(())
            .map(|_| {
                let future = get_ip();
                future.wait().expect("failed to successfully resolve future");
            });

        rt::run(f);
    }
}
