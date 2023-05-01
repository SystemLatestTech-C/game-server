use std::net::TcpListener;
use std::io;
use std::thread;
use std::io::{Read, Write};
use std::fs::OpenOptions;
use std::time::SystemTime;

pub const SERVER_ADDR: &str = "0.0.0.0:8080"; 

pub fn main() -> io::Result<()> {

    //파일 쓰기
    let mut file = OpenOptions::new()
        .append(true)
        .open("./logs/logs.txt")?;

    file.write_all(b"new data")?;

    let listener = TcpListener::bind(SERVER_ADDR)?;
    println!("Server listening on {}", SERVER_ADDR);

    loop {
        let (mut client_1_socket, client_1_addr) = listener.accept()?;
        println!("Client 1 connected from {}", client_1_addr);
        file.write_all(format!("player 1 : {}, at {:?}\n", client_1_addr, SystemTime::now()).as_bytes())?;

        let (mut client_2_socket, client_2_addr) = listener.accept()?;
        println!("Client 2 connected from {}", client_2_addr);
        file.write_all(format!("player 2 : {}, at {:?}\n", client_2_addr, SystemTime::now()).as_bytes())?;

        file.write_all(format!("game start!! at {:?} with player1({}) and player2({})\n", SystemTime::now(), client_1_addr, client_2_addr).as_bytes())?;


        let mut client_1_socket_clone = client_1_socket.try_clone()?;
        let mut client_2_socket_clone = client_2_socket.try_clone()?;

        let client_1_to_client_2 = thread::spawn(move || {
            let mut buffer = [0; 1024];
            loop {
                match client_1_socket.read_exact(&mut buffer) {

                    Ok(_) => {
                        println!("player1 sent : {:?}", buffer);
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
            let mut buffer = [0; 1024];
            loop {
                match client_2_socket_clone.read_exact(&mut buffer) {
                    Ok(_) => {
                        println!("player2 sent : {:?}", buffer);
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
        file.write_all(format!("game ended at {:?} with player1({}) and player2({})\n", SystemTime::now(), client_1_addr, client_2_addr).as_bytes())?;
    }

    Ok(())
}