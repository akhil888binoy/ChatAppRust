use std::fs::{read, read_to_string};
use std::net::{TcpListener , TcpStream};
use std::thread;
use std::io::{stdin, ErrorKind, Read, Write};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE : usize = 32 ;

fn handleclient(mut stream: TcpStream ,  clients: Arc<Mutex<Vec<TcpStream>>>) {
    let mut buffer = [0; MSG_SIZE];
    let peer_addr = stream.peer_addr().unwrap();
    println!("Client connected: {}", peer_addr);
    println!("{:?}" , stream);

    loop {
        match stream.read(&mut buffer){
            Ok(0)=>{
                println!("Client is disconnected");
                break;
            }
            Ok(bytesread)=>{
                let message = String::from_utf8_lossy(&buffer[..bytesread]);
                 let clients_lock = clients.lock().unwrap();
                 for client in clients_lock.iter() {
                    let mut clientmu = client;
                     if client.peer_addr().unwrap() != peer_addr {
                         let _ = clientmu.write_all(message.as_bytes());
                     }
                 }
            }
            Err(e)=>{     
                    println!("Error reading from client: {:?}", e);
                    break;               
            }
        }
    }
    let mut clients_lock = clients.lock().unwrap();
    clients_lock.retain(|client| client.peer_addr().unwrap() != peer_addr);
}


fn main() {
    let server = TcpListener::bind(LOCAL).expect("Failed to listen the port");
    println!("Server listening at port : {}" ,LOCAL );
    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));
    for stream in server.incoming(){
        match stream {
            Ok(stream)=>{
                println!("New client connected");                
                let clients_clone = Arc::clone(&clients);
                let mut clients_lock = clients.lock().unwrap();
                clients_lock.push(stream.try_clone().expect("Failed to clone stream"));
                thread::spawn(move || handleclient(stream, clients_clone));
            }
            Err(e)=>{
                println!("Failed to accept {:?}",e);
            }
        }
    }
}
