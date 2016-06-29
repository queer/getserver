use std::io::{Read, Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};
use std::fs::File;

const WEBSERVER_ROOT: &'static str = "root";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:15973").unwrap();
    loop {
        let stream_and_socket = listener.accept().unwrap();
        read_request(stream_and_socket.0);
    }
}

fn read_request(stream: TcpStream) {
    let mut lines: Vec<String> = vec![];
    {
        let mut reader = BufReader::new(&stream);
        for line in reader.by_ref().lines() {
            let line_unwrapped = line.unwrap();
            match line_unwrapped.as_str() {
                "" => { break; }
                _ => lines.push(line_unwrapped),
            }
        }
    }
    send_response(stream, lines);
}

fn send_response(mut stream: TcpStream, lines: Vec<String>) {
    let request_type_location = lines[0].split_whitespace();
    let mut request_initial_header: Vec<String> = vec![];
    for split in request_type_location {
        request_initial_header.push(split.into());
    }

    let mut response = String::from("HTTP/1.1 ");
    if request_initial_header[0] != "GET" {
        response.push_str("501 NOT IMPLEMENTED");
    } else if request_initial_header[2] != "HTTP/1.1" {
        response.push_str("505 HTTP VERSION NOT SUPPORTED");
    } else {
        let ref mut path = request_initial_header[1];
        // Automatically make / load index.html
        let final_header_path: String = (if path == "/" { "/index.html" } else { path }).into();
        let mut final_path: String = String::with_capacity(WEBSERVER_ROOT.len() + final_header_path.len());
        final_path.push_str(WEBSERVER_ROOT);
        final_path.push_str(final_header_path.as_str());
        let mut data = String::new();
        let file_result = File::open(final_path);
        match file_result {
            Ok(_) => {
                let mut file = file_result.unwrap();
                file.read_to_string(&mut data).unwrap();
                response.push_str("200 OK\n\n");
                response.push_str(data.as_str());
            },
            _ => {
                // TODO: Custom 404 page?
                response.push_str("404 NOT FOUND, GIVING UP AND CRYING :( PLEASE DON'T BE MEAN TO INNOCENT WEBSERVERS");
            },
        }
    }

    stream.write_all(response.as_bytes()).unwrap();
}

