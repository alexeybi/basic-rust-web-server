mod request;

use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

use crate::request::parse_request;
use log::{error, info};
use scoped_threadpool::Pool;
use std::{fs, thread};

pub fn listen_to_stream(listener: TcpListener) -> ! {
    let threads_num = num_cpus::get() as u32;
    let mut pool = Pool::new(threads_num);

    loop {
        let stream = listener.incoming().next().unwrap();
        let stream = stream.expect("Can't process incoming stream.");
        stream
            .set_read_timeout(Some(Duration::from_millis(10)))
            .expect("Can't set stream read timeout.");
        pool.scoped(|scope| {
            scope.execute(|| handle_connection(stream).expect("Can't handle connection"))
        });
    }
}

pub fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&mut buffer[..]);
    let request_line = request.lines().next().unwrap();

    match parse_request(&request_line) {
        Ok(r) => {
            info!("\nRequest: {}", &r);
            let content = fs::read_to_string("index.html").unwrap();
            let response = format!("{}{}", "HTTP/1.1 200 OK OK\r\n\r\n", content);
            info!("Response: {}", &response);
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(_) => {
            error!("Invalid request: {}", &request);
            let content = fs::read_to_string("404.html").unwrap();
            let response = format!("{}{}", "HTTP/1.1 404 Not found!\r\n\r\n", content);
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
    Ok(())
}
