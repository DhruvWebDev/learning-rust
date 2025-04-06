#[allow(unused_imports)]
use std::net::TcpListener;
use std::io::{BufRead, BufReader, Read, Write};

/*
/ Status line
HTTP/1.1  // HTTP version
200       // Status code
OK        // Optional reason phrase
\r\n      // CRLF that marks the end of the status line

// Headers (empty)
\r\n      // CRLF that marks the end of the headers

// Response body (empty)

*/
fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                // Create BufReader and clone request line safely
                let mut buf_reader = BufReader::new(&mut stream);
                let mut request_line = String::new();
                buf_reader.read_line(&mut request_line).unwrap();

                println!("request_line: {}", request_line.trim());

                let response = match request_line.trim() {
                    "GET / HTTP/1.1" => "HTTP/1.1 200 OK\r\n\r\n",
                    _ => "HTTP/1.1 404 Not Found\r\n\r\n",
                };

                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap(); // optional, but good practice
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
