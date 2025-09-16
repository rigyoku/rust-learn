use std::net::TcpListener;

fn main() {
    println!("Start listener!");
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        println!("get stream: {:?}", stream);
    }

    println!("Hello, world!");
}
