use std::{
    io::{prelude::*, BufReader, ErrorKind},
    net::{TcpListener, TcpStream},
    sync::mpsc,
    thread,
};

const MSG_SIZE: usize = 32;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    listener
        .set_nonblocking(true)
        .expect("Failed to initialize nonblocking server.");

    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<String>();
    loop {
        if let Ok((mut socket, address)) = listener.accept() {
            println!("Client connected: {}", address);

            let tx = tx.clone();
            clients.push(socket.try_clone().expect("Could not clone client."));

            thread::spawn(move || loop {
                let mut buffer = vec![0; MSG_SIZE];
                match socket.read_exact(&mut buffer) {
                    Ok(_) => {
                        let msg = buffer
                            .into_iter()
                            .take_while(|&x| x != 0)
                            .collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid msg.");

                        println!("{}: {:?}", address, msg);
                        tx.send(msg).expect("Failed to send message to rx");
                    }
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("Closing connection with: {}", address);
                        break;
                    }
                }
                thread::sleep(::std::time::Duration::from_millis(100));
            });
        }
        if let Ok(msg) = rx.try_recv() {
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    let mut buf = msg.clone().into_bytes();
                    buf.resize(MSG_SIZE, 0);

                    client.write_all(&buf).map(|_| client).ok()
                })
                .collect::<Vec<_>>();
        }
        thread::sleep(::std::time::Duration::from_millis(100));
    }
}
