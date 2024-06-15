use std::{io::Write, net::TcpListener};

fn main() {
    // creating a tcp listener
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                match stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes()) {
                    Ok(_) => {}
                    Err(e) => eprintln!("Failed to write stream {e}"),
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
