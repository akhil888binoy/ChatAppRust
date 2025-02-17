use std::io::{self, Read, Write, stdin};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::mpsc;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn senderClient(mut stream: TcpStream) {
    let mut input = String::new();
    loop {
        println!("Enter a message");
        input.clear();
        stdin().read_line(&mut input).expect("cannot read");
        let message = input.trim().to_string();
        
        if message.is_empty() {
            continue; 
        }
        stream.write_all(message.as_bytes()).expect("Failed to send message");
    }
}

fn receiverClient(mut stream: TcpStream) {
    let mut buffer = [0; MSG_SIZE];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Server disconnected.");
                break;
            }
            Ok(bytes_read) => {
                let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Server: {}", message);
            }
            Err(_) => {
                println!("Error reading from server.");
                break;
            }
        }
    }
}

fn main() {
    let stream = TcpStream::connect(LOCAL).expect("Cannot connect to server");
    let streamclone = stream.try_clone().expect("Failed to clone stream");

    thread::spawn(move || senderClient(streamclone)); 
    thread::spawn(move || receiverClient(stream)); 

    loop {
        thread::sleep(std::time::Duration::from_secs(60));
    }
}
