use std::env;
use std::io::{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process;
use std::str;
use std::fs;
use std::thread;
use std::time::Duration;
use web_server::ThreadPool;

const DEF_IP: &str = "127.0.0.1";
const DEF_PORT: &str = "8080";
const BUFF_SIZE: usize = 4096;

fn gen_addr() -> String {
    let args: Vec<String> = env::args().collect();

    let ip = match args.get(1) {
        Some(s) => s,
        None => DEF_IP,
    };
    let port = match args.get(2) {
        Some(s) => s,
        None => DEF_PORT,
    };

    format!("{}:{}", ip, port)
}

fn bind(addr: &str) -> TcpListener {
    let listener = TcpListener::bind(addr);
    match listener {
        Ok(l) => {
            println!("Listening to {}", addr);
            l
        }
        Err(e) => match e.kind() {
            ErrorKind::PermissionDenied => {
                eprintln!("Permission Denied for binding to {}", addr);
                process::exit(1);
            }
            ErrorKind::AddrInUse => {
                eprintln!("Address {} Already in Use.", addr);
                eprintln!("Please try a different Port");
                process::exit(1);
            }
            _ => {
                eprintln!("Cannot bind to {}", addr);
                eprintln!("Make sure the IP and Port number given is correct");
                println!("Usage: web_server <IP_Address> <Port>");
                process::exit(1);
            }
        },
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buff = [0; BUFF_SIZE];
    let mut http_request = String::new();
    loop {
        let n = match stream.read(&mut buff) {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Cannot Read from TcpStream");
                return;
            }
        };
        if n == 0 { break }
        let buff_str = match str::from_utf8(&buff[..n]) {
            Ok(s) => s,
            Err(_) => {
                eprintln!("bytes in buffer are not utf8");
                return;
            }
        };
        http_request.push_str(buff_str);

        if http_request.ends_with("\r\n\r\n") { break }
    }
    // println!("request:\n{}", http_request);

    let (filename, status_code) = match http_request.split(" ").skip(1).next().unwrap() {
        "/" => ("index.html", "200 OK"),
        "/sleep" => {
            thread::sleep(Duration::from_secs(5));
            ("index.html", "200 OK")
        }
        _ => ("404.html", "404 NOT FOUND")
    };

    let html = match fs::read_to_string("../".to_owned() + filename) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Error Reading file {}", filename);
            return
        }
    };

    let mut headers = Vec::new();
    headers.push(String::from("Content-Type: text/html; charset=utf-8\r\n"));
    headers.push(format!("Content-Length: {}\r\n", html.len()));
    let mut header = String::new();
    for head in headers {
        header.push_str(&head);
    }
    header.push_str("\r\n");

    let status_line = "HTTP/1.1 ".to_owned() + status_code + "\r\n";

    let response = format!("{}{}{}", status_line, header, html);
    let response = response.as_bytes();

    match stream.write_all(response) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Cannot send response.");
        }
    };
}


fn main() {
    let addr = gen_addr();
    let listener = bind(&addr);
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Connection Failed");
                continue;
            }
        };
        println!("Connection established! {:?}", stream);

        pool.add(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.");
}
