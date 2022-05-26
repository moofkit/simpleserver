use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::time::Duration;
use std::thread;
use simpleserver::ThreadPool;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(100);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(||{
            handle_connection(stream);
        })
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..n]));

    let root = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status, file_path) = if buffer.starts_with(root) {
        ("200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("200 OK", "hello.html")
    } else {
        ("404 Not Found", "404.html")
    };
    let content = fs::read_to_string(file_path).unwrap();
    let response = format!("HTTP/1.1 {}\r\nContent-Length:{}\r\n\r\n{}", status, content.len(), content);
    println!("Response: {}", response);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
