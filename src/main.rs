#[warn(unused_variables)]

use std::io::prelude::*;
// importing TCP listener
use std::net::{TcpListener, TcpStream};
// fs module
use std::fs;


fn main() {
    // creating a TCP listener
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // the buffer storing the request data
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    // reading contents from the `index.html` file using the fs module
    let contents = fs::read_to_string("index.html").unwrap();

    // hard coding the first line of the response
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    // write the response back to the client
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
