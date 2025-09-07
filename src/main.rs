use std::net::{TcpListener, TcpStream};
use std::{io::prelude::*, fs, env};
use colored::*;

fn main() {
    // gets user input
    let args: Vec<String> = env::args().collect();
    if args.len() > 1   {
        ok(args);
    } else {
        help();
    }

}

fn help() {
    println!("rstp | a simple HTTP server written in Rust 🦀");
    println!("");
    println!("usage:");
    println!("");
    println!("rstp <file> <port>");
}

fn ok(args: Vec<String>) {
    // colors
    let d_yellow = Color::TrueColor { r:255, g:242, b:102 };
    let br_yellow = Color::TrueColor {r:250, g:246, b:202 };
    // checks default file
    match fs::read_to_string(&args[1]) {
        Ok(file) => {
            // attempts to initialize server
            let server = match TcpListener::bind(format!("127.0.0.1:{}", &args[2])) {
                Ok(server) => {
                    println!("{} {} {} {}", "running server on:".color(d_yellow), "127.0.0.1".color(br_yellow), "on port:".color(d_yellow), &args[2].color(br_yellow));
                    println!("");
                    server
                },
                Err(_) => {
                    println!("Error with server port");
                    help();
                    return;
                }
            };

            for connection in server.incoming() {
                match connection {
                    Ok(connection) => {
                        handle_connection(connection, &file);
                    }
                    Err(e) => {
                        println!("Encountered an error while handling connection: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Encountered an error while handling file: {}", e);
            help();
        }
    }
}

fn handle_connection(mut stream: TcpStream, defaultfile: &String) {
    // takes inbound connections and gets requested file, method, host
    // and converts them for use
    let mut buffer = [0; 1024];
    let buf = stream.read(&mut buffer).unwrap();
    let buf_str = String::from_utf8_lossy(&buffer[..buf]);
    let buf_vec: Vec<&str> = buf_str.split_whitespace().collect();
    let requested_file = &buf_vec[1];
    let method = &buf_vec[0];
    let host = buf_vec[4];
    // displays connection information in terminal
    let br_yellow = Color::TrueColor { r: 250, g: 246, b: 202};
    let d_yellow = Color::TrueColor { r: 255, g: 242, b: 102};
    println!("{}", "Received Connection:".color(d_yellow));
    println!("{} {}", "Host:".color(d_yellow), host.color(br_yellow));
    println!("{} {}", "Method:".color(d_yellow), method.color(br_yellow));
    println!("{} {}", "Requested File:".color(d_yellow), requested_file.color(br_yellow));
    println!("");
    let filtered_file = &requested_file[1..];
    let header = "HTTP/1.1 200 OK\r\n\r\n";
    let file = if *requested_file == "/" {
        defaultfile.clone()
    } else {
        let f = match fs::read_to_string(filtered_file) {
            Ok(f) => f,
            Err(_) => defaultfile.clone()
        };
        f
    };
    let response = format!(
        "{} {}",
        header,
        file
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
