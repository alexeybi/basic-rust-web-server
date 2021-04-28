use core::fmt;
use std::env;
use std::error::Error;
use std::path::Path;

pub struct Request<'a> {
    method: &'a str,
    uri: &'a Path,
    http_version: &'a str,
}

impl<'a> fmt::Display for Request<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}\r\n",
            self.method,
            self.uri.display(),
            self.http_version,
        )
    }
}
pub fn parse_request(request: &str) -> Result<Request, Box<dyn Error>> {
    let root: String = env::var("ROOT").unwrap_or("".to_string());

    let mut request_split = request.split_whitespace();

    let method = request_split.next().ok_or("Method not found")?;
    if method != "GET" {
        Err("Method is not supported")?;
    }
    let uri = Path::new(request_split.next().ok_or("URI is missing")?);
    let _uri = uri.to_str().expect("Error converting URI to string");

    let http_version = request_split.next().ok_or("HTTP version is missing")?;
    if http_version != "HTTP/1.1" {
        Err("Unsupported HTTP version, HTTP/1.1 is expected.")?;
    }

    if !Path::new(&format!("{}{}", root, _uri)).exists() {
        Err("Resource does not exist")?;
    }
    Ok(Request {
        method,
        uri,
        http_version,
    })
}
