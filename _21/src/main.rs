// use std::{fs, io::{BufRead, BufReader, Write}, net::TcpListener, thread};

// fn main() {
//     println!("Start listener!");
//     let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

//     for stream in listener.incoming() {
//         let mut _stream = stream.unwrap();
//         let reader = BufReader::new(&_stream);
//         let vec: Vec<_> = reader.lines().map(|line|line.unwrap()).take_while(|line|!line.is_empty()).collect();
//         println!("Request: {:#?}", vec);
//         if vec[0].starts_with("GET / HTTP/1.1") {
//             let head = "HTTP/1.1 200 OK\r\n";
//             let body = fs::read_to_string("/Applications/htc-3/self/rust-learn/_21/res/index.html").unwrap();
//             let len = body.len();
//             let response = format!("{}Content-Length: {}\r\nContent-Type: text/html\r\n\r\n{}", head, len, body);
//             _stream.write_all(response.as_bytes()).unwrap();
//         } else if vec[0].starts_with("GET /sleep HTTP/1.1") {
//             let head = "HTTP/1.1 200 OK\r\n";
//             let body = fs::read_to_string("/Applications/htc-3/self/rust-learn/_21/res/index.html").unwrap();
//             let len = body.len();
//             thread::sleep(std::time::Duration::from_secs(5));
//             let response = format!("{}Content-Length: {}\r\nContent-Type: text/html\r\n\r\n{}", head, len, body);
//             _stream.write_all(response.as_bytes()).unwrap();
//         } else {
//             let head = "HTTP/1.1 404 NOT FOUND\r\n";
//             let body = fs::read_to_string("/Applications/htc-3/self/rust-learn/_21/res/404.html").unwrap();
//             let len = body.len();
//             let response = format!("{}Content-Length: {}\r\nContent-Type: text/html\r\n\r\n{}", head, len, body);
//             _stream.write_all(response.as_bytes()).unwrap();
//         }
//     }

//     println!("Hello, world!");
// }

use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    thread,
};

use _21::ThreadPool;

fn main() {
    println!("Start listener!");
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let mut _stream = stream.unwrap();
        pool.execute(move || {
            let reader = BufReader::new(&_stream);
            let vec: Vec<_> = reader
                .lines()
                .map(|line| line.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();
            // println!("Request: {:#?}", vec);
            if vec.is_empty() {
                println!("Empty request received, skipping...");
            } else {
                if (&vec[0]).starts_with("GET / HTTP/1.1") {
                    let head = "HTTP/1.1 200 OK\r\n";
                    let body = fs::read_to_string(
                        "/Applications/htc-3/self/rust-learn/_21/res/index.html",
                    )
                    .unwrap();
                    let len = body.len();
                    let response = format!(
                        "{}Content-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
                        head, len, body
                    );
                    _stream.write_all(response.as_bytes()).unwrap();
                } else if (&vec[0]).starts_with("GET /sleep HTTP/1.1") {
                    let head = "HTTP/1.1 200 OK\r\n";
                    let body = fs::read_to_string(
                        "/Applications/htc-3/self/rust-learn/_21/res/index.html",
                    )
                    .unwrap();
                    let len = body.len();
                    thread::sleep(std::time::Duration::from_secs(5));
                    let response = format!(
                        "{}Content-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
                        head, len, body
                    );
                    _stream.write_all(response.as_bytes()).unwrap();
                } else {
                    let head = "HTTP/1.1 404 NOT FOUND\r\n";
                    let body =
                        fs::read_to_string("/Applications/htc-3/self/rust-learn/_21/res/404.html")
                            .unwrap();
                    let len = body.len();
                    let response = format!(
                        "{}Content-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
                        head, len, body
                    );
                    _stream.write_all(response.as_bytes()).unwrap();
                }
            }
        });
    }
    println!("Hello, world!");
}
