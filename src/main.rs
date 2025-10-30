use std::{
    io::{prelude::*},
    net::{TcpListener, TcpStream},
};
use mime_guess::from_path;
use colored::{Color, Colorize};

#[derive(Debug)]
struct Request {
    method: String,
    path: String,
}

fn cla() -> (String, bool) {
    let mut verbose = false;
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        help();
        std::process::exit(1);
    }
    if args.len() == 3 {
        match &args[2] as &str {
            "-v" | "--verbose" => verbose = true,
            _ => (),
        }
    }
    (args[1].clone(), verbose)
}

fn help() {
    println!("rstp | a simple HTTP server written in Rust 🦀\n");
    println!("usage:\n");
    println!("rstp <port>               | starts server on given port");
    println!("rstp <port> -v, --verbose | starts server on given port and displays extra information\n");
}

fn main() {
    let (port, verbose) = cla();
    let listener = match TcpListener::bind(format!("127.0.0.1:{port}")) {
        Ok(listener) => listener,
        Err(e) => {
            println!("error initializing listener at 127.0.0.1:{port}");
            println!("{e}");
            std::process::exit(1);
        }
    };

    let d_yellow = Color::TrueColor { r: 255, g: 242, b: 102 };
    let br_yellow = Color::TrueColor { r: 250, g: 246, b: 202 };
    println!(
        "{} {} {} {}\n",
        "running server on:".color(d_yellow),
        "127.0.0.1".color(br_yellow),
        "on port:".color(d_yellow),
        port.color(br_yellow)
    );

    for connection in listener.incoming() {
        let mut connection = match connection {
            Ok(conn) => conn,
            Err(_) => continue,
        };

        let request = handle_connection(&mut connection);
        let (file, body, mtype) = parse(&request.path);
        let ip = match connection.peer_addr() {
            Ok(addr) => addr.ip().to_string(),
            Err(_) => "unavailable".to_string(),
        };

        let header = "HTTP/1.1 200 OK\r\n";
        let rmtype = format!("Content-Type: {mtype}\r\n");
        let len = format!("Content-Length: {}\r\n", body.len());
        let response = format!("{header}{rmtype}{len}\r\n");

        display(&request, &file, &mtype, ip, verbose);

        let _ = connection.write_all(response.as_bytes());
        let _ = connection.write_all(&body);
        let _ = connection.flush();
    }
}

fn handle_connection(stream: &mut TcpStream) -> Request {
    let mut buf = [0; 4096];
    let n = match stream.read(&mut buf) {
        Ok(n) => n,
        Err(_) => 0,
    };

    let request_text = match std::str::from_utf8(&buf[..n]) {
        Ok(text) => text,
        Err(_) => "",
    };

    let first_line = request_text.lines().next().unwrap_or("GET /error.html");
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("GET").to_string();
    let mut path = parts.next().unwrap_or("/").to_string();
    if path == "/" { path = "index.html".to_string(); }

    Request { method, path }
}

fn display(input: &Request, file: &str, mt: &str, ip: String, verbose: bool) {
    let d_yellow = Color::TrueColor { r: 255, g: 242, b: 102 };
    let br_yellow = Color::TrueColor { r: 250, g: 246, b: 202 };
    println!("{}", "Received Connection:".color(d_yellow));
    println!("{}: {}", "From".color(d_yellow), ip.color(br_yellow));
    println!("{}: {}", "Method".color(d_yellow), input.method.color(br_yellow));
    println!("{}: {}", "Requested Path".color(d_yellow), input.path.color(br_yellow));
    if verbose {
        println!("{}: {}", "Given Path".color(d_yellow), file.color(br_yellow));
        println!("{}: {}", "MIME Type".color(d_yellow), mt.color(br_yellow));
    }
    println!();
}

fn parse(path: &str) -> (String, Vec<u8>, String) {
    let mut file = path.to_string();
    let output = match std::fs::read(&file) {
        Ok(output) => output,
        Err(_) => match std::fs::read("error.html") {
            Ok(err) => {
                file = "error.html".to_string();
                err
            }
            Err(_) => {
                let fallback = b"There was an error processing your request";
                file = "error.html".to_string();
                fallback.to_vec()
            }
        },
    };
    let mimetype = from_path(&file).first_or_octet_stream();
    (file, output, mimetype.to_string())
}

