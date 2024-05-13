use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

use web_server::ThreadPool;

fn main() 
{
    // bind returns result, in Production, we want to include error handling
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let thread_pool = ThreadPool::new(4);

    for stream in listener.incoming()
    {
        let stream = stream.unwrap();
        println!("connection has been established");

        // similar to `thread::spawn()` but limited to 4 threads instead of inifite
        thread_pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) 
{
    // buffer to hold data that is read in Prod, we would handle buffer of any size
    let mut buffer = [0; 1024];

    // `read` takes in mutable reference to self 
    stream.read(&mut buffer).unwrap();

    // hard-coded request we are expecting
    // `b`` prefix will return byte array representing string
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get)
    {
        ("HTTP/1.1 200 OK", "index.html")
    } 
    else if buffer.starts_with(sleep) 
    {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    } 
    else 
    {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
