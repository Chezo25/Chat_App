use std::io:: {ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
    let server = TcpListener::bind(LOCAL).expect("Listener to fail");
    server.set_nonblocking(true).expect("failed to initialize block"):

    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<String>();
    loop {
        if let ok((mut socket, addr)) = server.accept() { //  this allows the connection to this server
            println!("Client {} connected", addr );
            let tx = tx.clone();
        }
    }
}
