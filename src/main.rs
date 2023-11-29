// Uncomment this block to pass the first stage
use std::{
    io::Read,
    io::Write,
    net::{TcpListener, TcpStream},
    thread,
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
                thread::spawn(move || handle_connection(stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024]; // max size of 1024 bytes
    stream.read(&mut buffer)?;

    let request = String::from_utf8_lossy(&buffer[..]);
    request.lines().for_each(|line| println!("{}", line));
    let mut lines = request.lines();
    let first_line = lines.next().unwrap();
    let _host = lines.next().unwrap();
    let user_agent_full = lines.next().unwrap();
    let user_agent = user_agent_full.split(" ").nth(1).unwrap();
    let _accept = lines.next().unwrap();
    let path = first_line.split_whitespace().nth(1).unwrap();

    if path == "/" {
        stream.write(b"HTTP/1.1 200 OK\r\n\r\n")?;
    } else if path.starts_with("/echo/") {
        let echo = path.split("/echo/").nth(1).unwrap();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            echo.len(),
            echo
        );
        stream.write(response.as_bytes())?;
    } else if path.starts_with("/user-agent") {
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            user_agent.len(),
            user_agent
        );
        stream.write(response.as_bytes())?;
    } else {
        stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n")?;
    }

    Ok(())
}
