use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};
use mime_guess::from_path;
use colored::{Color, Colorize};

#[derive (Debug)]
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
            _ => ()
        }
    }
    return (args[1].clone(), verbose);
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
    let d_yellow = Color::TrueColor { r:255, g:242, b:102 };
    let br_yellow = Color::TrueColor {r:250, g:246, b:202 };
    println!("{} {} {} {}\n", "running server on:".color(d_yellow), "127.0.0.1".color(br_yellow), "on port:".color(d_yellow), port.color(br_yellow));
    for connection in listener.incoming() {
        let mut connection = connection.unwrap();
        let request = handle_connection(connection.try_clone().expect("Error reading connection"));
        let (file, body, mtype) = parse(&request.path);
        let ip = match connection.peer_addr() {
            Ok(addr) => addr.ip().to_string(),
            Err(_) => "unavailable".to_string()
        };
        let header = "HTTP/1.1 200 OK\r\n";
        let rmtype = format!("Content-Type: {mtype}\r\n");
        let len = format!("Content-Length: {}\r\n", body.len());
        let response = format!("{header}{rmtype}{len}\r\n");
        display(&request, &file, &mtype, ip, verbose);
        let _ = connection.write(response.as_bytes());
        let _ = connection.write_all(&body);
        let _ = connection.flush();
    }
}

fn handle_connection(stream: TcpStream) -> Request {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    if http_request.len() < 1 {
        let output: Request = Request {
            method: "GET".to_string(),
            path: "/error.html".to_string()
        };
        return output;
    }
    let mut m: String = String::new();
    for i in http_request[0].chars() {
        if i == ' ' { break; }
        m.push(i);
    }
    let mut p: String = String::new();
    for i in http_request[0][(m.len() + 2)..].chars() {
        if i == ' ' { break; }
        p.push(i);
    }
    if p.trim() == "" { p = "index.html".to_string(); }
    let output: Request = Request {
        method: m,
        path: p,
    };
    return output;
}

fn display(input: &Request, file: &str, mt: &str, ip: String, verbose: bool) {
    let d_yellow = Color::TrueColor { r:255, g:242, b:102 };
    let br_yellow = Color::TrueColor {r:250, g:246, b:202 };
    println!("{}", "Received Connection:".color(d_yellow));
    print!("{}: {}\n", "From".color(d_yellow), ip.color(br_yellow));
    print!("{}: {}\n", "Method".color(d_yellow), input.method.color(br_yellow));
    print!("{}: {}\n", "Requested Path".color(d_yellow), input.path.color(br_yellow));
    if verbose {
        print!("{}: {}\n", "Given Path".color(d_yellow), file.color(br_yellow));
        print!("{}: {}\n", "MIME Type".color(d_yellow), mt.color(br_yellow));
    }
    println!();
}

fn parse(path: &str) -> (String, Vec<u8>, String){
    //let mut file: String = String::new();
    let mut file = path.to_string();
    let output = match std::fs::read(&file) {
        Ok(output) => output,
        Err(_) => {
            let error = match std::fs::read("error.html") {
                Ok(error) => error,
                Err(_) => {
                    let m: &[u8] = b"There was an error processing your request";
                    let vec: Vec<u8> = m.to_vec();
                    file = "error.html".to_string();
                    vec
                }
            };
            error
        },
    };
    let mimetype = from_path(&file).first_or_octet_stream();
    return (file, output, mimetype.to_string());
}
