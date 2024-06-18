use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 256];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);

    let tokens: Vec<&str> = request.split(" ").collect();

    match tokens[0] {
        "GET" => {
            if tokens[1] == "/" {
                stream.write(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
            } else if tokens[1].starts_with("/echo/") {
                let response = tokens[1].replace("/echo/", "");
                let content_type = "Content-Type: text/plain\r\n";
                let content_len = response.len();
                stream
                    .write(
                        format!(
                            "HTTP/1.1 200 OK\r\n{}Content-Length: {}\r\n\r\n{}",
                            content_type, content_len, response
                        )
                        .as_bytes(),
                    )
                    .unwrap();
            } else {
                stream
                    .write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                    .unwrap();
            }
        }

        _ => {
            eprintln!("Unknown method: {}", tokens[0]);
        }
    }

    stream.flush().unwrap();
}

fn main() {
    // creating a tcp listener
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                handle_connection(_stream);
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
}
