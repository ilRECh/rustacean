use std::{
    fs,
    io::{ prelude::*, BufReader },
    net::{ TcpListener, TcpStream },
    thread,
    time::Duration
};

use webserv::ThreadPool;

fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();
    let pool = ThreadPool::build(4).expect("The size should be > 0");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello_world.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(30));
            ("HTTP/1.1 200 OK", "hello_world.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{status_line}\r\nContent-Length: {}\r\n\r\n{contents}", contents.len());

    stream.write_all(response.as_bytes()).unwrap();
}
