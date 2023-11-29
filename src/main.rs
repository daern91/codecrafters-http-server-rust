use std::{
    env, fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let directory = args.get(2).unwrap_or(&String::from(".")).to_string();

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let dir_clone = directory.clone();
                thread::spawn(move || handle_connection(stream, dir_clone).unwrap());
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream, directory: String) -> std::io::Result<()> {
    let mut buffer = [0; 10240];
    stream.read(&mut buffer)?;

    let request = String::from_utf8_lossy(&buffer);
    let mut lines = request.lines();
    let first_line = lines.next().unwrap_or_default();
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or_default();
    let path = parts.next().unwrap_or_default();

    match path {
        "/" => stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n")?,
        _ if path.starts_with("/echo/") => handle_echo(&mut stream, path)?,
        "/user-agent" => handle_user_agent(&mut stream, &mut lines)?,
        _ if path.starts_with("/files/") => {
            handle_files(&mut stream, path, method, &directory, &mut lines)?
        }
        _ => stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n")?,
    }

    Ok(())
}

fn handle_echo(stream: &mut TcpStream, path: &str) -> std::io::Result<()> {
    let echo_message = path.split("/echo/").nth(1).unwrap_or_default();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        echo_message.len(),
        echo_message
    );
    stream.write_all(response.as_bytes())
}

fn handle_user_agent(stream: &mut TcpStream, lines: &mut std::str::Lines) -> std::io::Result<()> {
    let user_agent = lines
        .nth(1)
        .unwrap_or_default()
        .split(' ')
        .nth(1)
        .unwrap_or_default();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        user_agent.len(),
        user_agent
    );
    stream.write_all(response.as_bytes())
}

fn handle_files(
    stream: &mut TcpStream,
    path: &str,
    method: &str,
    directory: &str,
    lines: &mut std::str::Lines,
) -> std::io::Result<()> {
    let file_path = path.split("/files/").nth(1).unwrap_or_default();

    match method {
        "GET" => {
            let full_path = format!("{}/{}", directory, file_path);
            match fs::read_to_string(&full_path) {
                Ok(file_contents) => {
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
                        file_contents.len(),
                        file_contents
                    );
                    stream.write_all(response.as_bytes())
                }
                Err(_) => stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n"),
            }
        }
        "POST" => {
            let full_path = format!("{}/{}", directory, file_path);
            let mut file = fs::File::create(full_path)?;
            let body = lines.last().unwrap_or_default().trim_end_matches('\x00');
            file.write_all(body.as_bytes())?;
            stream.write_all(b"HTTP/1.1 201 Created\r\n\r\n")
        }
        _ => stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n"),
    }
}
