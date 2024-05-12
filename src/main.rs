use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() 
{
    // bind returns result, in Production, we want to include error handling
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming()
    {
        let stream = stream.unwrap();
        println!("connection has been established");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) 
{
    // buffer to hold data that is read in Prod, we would handle buffer of any size
    let mut buffer = [0; 1024];

    // `read` takes in mutable reference to self 
    stream.read(&mut buffer).unwrap();

    // convert slice of bytes to string
    println!("request: {}", String::from_utf8_lossy(&buffer[..]));

    /*
    Response needs to contain:
    - HTTP-Version Status-Code Reason-Phrase CRLF
    - message-body
    - e.g. HTTP/1.1 200 OK\r\n\r\n
    */
    
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
