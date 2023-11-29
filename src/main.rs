// Uncomment this block to pass the first stage
use std::{
    io::Read,
    io::Write,
    net::{TcpListener, TcpStream},
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                let _ = handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024]; // max size of 1024 bytes
    if let Ok(_) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer[..]);
        request.lines().for_each(|line| println!("{}", line));
        let mut lines = request.lines();
        let first_line = lines.next().unwrap();
        let path = first_line.split_whitespace().nth(1).unwrap();
        if path == "/" {
            let response = "HTTP/1.1 200 OK\r\n\r\n";
            stream.write(response.as_bytes())?;
        } else {
            let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
            stream.write(response.as_bytes())?;
        }
    }

    Ok(())
}
