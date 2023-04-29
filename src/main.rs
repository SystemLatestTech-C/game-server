use std::net::TcpListener;
use std::io;
use std::thread;
use std::io::{Read, Write};

pub const SERVER_ADDR: &str = "127.0.0.1:8080"; 

pub fn main() -> io::Result<()> {

    let listener = TcpListener::bind(SERVER_ADDR)?;
    println!("Server listening on {}", SERVER_ADDR);

    let (mut client_1_socket, client_1_addr) = listener.accept()?;
    println!("Client 1 connected from {}", client_1_addr);

    let (mut client_2_socket, client_2_addr) = listener.accept()?;
    println!("Client 2 connected from {}", client_2_addr);

    let mut client_1_socket_clone = client_1_socket.try_clone()?;
    let mut client_2_socket_clone = client_2_socket.try_clone()?;

    let client_1_to_client_2 = thread::spawn(move || {
        let mut buffer = [0; 12];
        loop {
            match client_1_socket.read_exact(&mut buffer) {
                Ok(_) => {
                    if let Err(e) = client_2_socket.write_all(&buffer) {
                        println!("Error sending data to client 2: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    println!("Error reading data from client 1: {}", e);
                    break;
                }
            }
        }
    });

    let client_2_to_client_1 = thread::spawn(move || {
        let mut buffer = [0; 4];
        loop {
            match client_2_socket_clone.read_exact(&mut buffer) {
                Ok(_) => {
                    if let Err(e) = client_1_socket_clone.write_all(&buffer) {
                        println!("Error sending data to client 1: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    println!("Error reading data from client 2: {}", e);
                    break;
                }
            }
        }
    });

    client_1_to_client_2.join().unwrap();
    client_2_to_client_1.join().unwrap();

    Ok(())
}