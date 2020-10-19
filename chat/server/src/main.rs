use std::io::{ErrorKind, Read, Write};
use std::net::Tcplistener;
use std::sync::mpsc;
use std::thread;

const local : &str = "localhost:8000";
const msg_size : usize= 32;

fn main(){
    let server = Tcplistener::bind(LOCAL).expect("Failed to bind");
    server.set_nonblocking(true).expect("failed to initialize non-blocking mode");

    let mut client = vec![];            // mutable vector   
    let (tx, rx) = mpsc::channel::<String>();

    loop {
        if let ok((mut socket, addre)) = server.accept() {
            println!("Client {} Connected!!", addre);

            let tx = tx.clone();
            clients.push(socket.try_clone().expect("Failed to clone the client"));

            thread::spawn(move || loop {
                let mut buff = vec![0; msg_size];
            })
        }
    }
}