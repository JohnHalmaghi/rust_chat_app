use std::io::Write;
use std::net::TcpStream;

fn main() {
    loop {
        chat();
    }
}

fn chat() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Failed to connect to server");
    let mut msg: String = String::new();

    std::io::stdin()
        .read_line(&mut msg)
        .expect("Couldn't read input.");

    stream
        .write_all(msg.as_bytes())
        .expect("Couldn't write to stream.");

    let _ = stream.flush();
}
