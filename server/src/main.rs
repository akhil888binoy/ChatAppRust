use std::fs::{read, read_to_string};
use std::net::{TcpListener , TcpStream};
use std::thread;
use std::io::{stdin, ErrorKind, Read, Write};
use std::sync::mpsc;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE : usize = 32 ;


fn handleclient(mut stream: TcpStream ) {
    let mut buffer = [0; MSG_SIZE];
    println!("{:?}" , stream);
    loop {
        match stream.read(&mut buffer){
            Ok(0)=>{
                println!("Client is disconnected");
                break;
            }
            Ok(bytesread)=>{
                let message = String::from_utf8_lossy(&buffer[..bytesread]);
                stream.write_all(message.as_bytes()).expect("Failed to send message"); 
                println!("Server Received: {}", message);
            }
            Err(e)=>{     
                    println!("Error reading from client: {:?}", e);
                    break;               
            }
        }

    }
}



fn main() {
    let server = TcpListener::bind(LOCAL).expect("Failed to listen the port");
    println!("Server listening at port : {}" ,LOCAL );
    for stream in server.incoming(){
        match stream {
            Ok(stream)=>{
                println!("New client connected");
                thread::spawn(move || handleclient(stream));
            }
            Err(e)=>{
                println!("Failed to accept {:?}",e);
            }
        }
    }
}
