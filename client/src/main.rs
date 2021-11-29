use std::io:: {self,ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;



fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("Stream to fail");
    client.set_nonblocking(true).expect("failed to init non blocking");

    let (tx, rx) = mpsc::channel::<String>();
    
    thread::spawn(move || loop{
        let mut buff = vec![0; MSG_SIZE];
        match socket.read_exact(&mut buff) {
            ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x = 0).collect::<Vec<_>>();
                println!("message recv {:?}", msg );
            },
            Err(ref err) if err.kind() = ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("connection with server was severed", );
                break;
            }
        }
        match rx.try_recv() {
            ok(msg) => 
        }    
        
    });
}
