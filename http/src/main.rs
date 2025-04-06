// use std::net::TcpListener;
// use std::io::{self, BufRead, BufReader, Write};
// use std::net::TcpStream;

// fn main() {
//     println!("Logs from your program will appear here!");

//     let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

//     for stream in listener.incoming() {
//         match stream {
//             Ok(stream) => {
//                 std::thread::spawn(move || {
//                     handle_connection(stream).unwrap();
//                 });
//             }
//             Err(e) => {
//                 println!("error: {}", e);
//             }
//         }
//     }
// }

// fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
//     println!("accepted new connection");

//     let mut buf_reader = BufReader::new(&mut stream);
//     let mut request_line = String::new();
//     buf_reader.read_line(&mut request_line)?;

//     let trimmed = request_line.trim();
//     let path = trimmed.split_whitespace().nth(1).unwrap_or("/");
//     println!("path: {}", path);

//     let echo_value = path.split('/').nth(2).unwrap_or("");
//     println!("value to echo: {}", echo_value);
//     println!("request_line: {}", trimmed);

//     let response = if trimmed == "GET / HTTP/1.1" {
//         "HTTP/1.1 200 OK\r\n\r\n".to_string()
//     } else if trimmed == format!("GET /echo/{} HTTP/1.1", echo_value) {
//         format!(
//             "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
//             echo_value.len(),
//             echo_value
//         )
//     } else {
//         "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
//     };

//     stream.write_all(response.as_bytes())?;
//     stream.flush()?;
//     Ok(())
// }

use std::net::TcpListener;
use std::io::{BufRead, BufReader, Write};

fn main() {
    println!("Server running on 127.0.0.1:4221");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Accepted new connection");

                let mut buf_reader = BufReader::new(&mut stream);
                let mut request_line = String::new();
                buf_reader.read_line(&mut request_line).unwrap();

                let trimmed = request_line.trim();
                let path = trimmed.split_whitespace().nth(1).unwrap_or("/");
                println!("Path: {}", path);

                let echo_value = path.split('/').nth(2).unwrap_or("");
                println!("Echoing: {}", echo_value);

                let response = if trimmed == "GET / HTTP/1.1" {
                    "HTTP/1.1 200 OK\r\n\r\n".to_string()
                } else if trimmed == format!("GET /echo/{} HTTP/1.1", echo_value) {
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                        echo_value.len(),
                        echo_value
                    )
                } else {
                    "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
                };

                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }
}
