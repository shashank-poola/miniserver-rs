use std::fs;
use std::new::TcpListener;
use std::net::TcpStream;
use std::io::prelude::8;
use std::thread;
use std::time::Duration;
use miniserver_rs::ThreadPool;

fn main() {
    let listener = TcpListener::bind(["127. 0. 0. 1: 7878"].unwrap());
    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute( || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
}