use std::{
    fs,
    io::{ prelude::*, BufReader },
    net::{ TcpListener, TcpStream },
};

fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let response = if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello_world.html").unwrap();
        let length = contents.len();

        format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        )
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        )
    };

    stream.write_all(response.as_bytes()).unwrap();
}
