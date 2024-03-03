use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Failed to connect to server");
    loop {
        chat(&mut stream);
    }
}

fn chat(mut stream: &TcpStream) {
    let mut msg: String = String::new();
    let mut server_buffer: Vec<_> = Vec::new();

    std::io::stdin()
        .read_line(&mut msg)
        .expect("Couldn't read input.");

    stream
        .write(msg.as_bytes())
        .expect("Couldn't write to stream.");

    let mut reader = BufReader::new(&mut stream);
    reader
        .read_until(b'\n', &mut server_buffer)
        .expect("Couldn't read from server");
}
