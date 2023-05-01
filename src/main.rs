use chrono::{format, Local};
use std::fs::OpenOptions;
use std::io;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;

pub const SERVER_ADDR: &str = "0.0.0.0:8080";

pub fn main() -> io::Result<()> {
    //파일 쓰기
    let mut file = OpenOptions::new().append(true).open("./logs/logs.txt")?;

    let listener = TcpListener::bind(SERVER_ADDR)?;
    println!("Server listening on {}", SERVER_ADDR);
    let score = [0, 0];
    let score = Arc::new(Mutex::new(score));

    loop {
        let (mut client_1_socket, client_1_addr) = listener.accept()?;
        println!("Client 1 connected from {}", client_1_addr);
        file.write_all(
            format!("player 1 : {}, at {:?}\n", client_1_addr, Local::now()).as_bytes(),
        )?;

        let (mut client_2_socket, client_2_addr) = listener.accept()?;
        println!("Client 2 connected from {}", client_2_addr);
        file.write_all(
            format!("player 2 : {}, at {:?}\n", client_2_addr, Local::now()).as_bytes(),
        )?;

        file.write_all(
            format!(
                "game start!! at {:?} with player1({}) and player2({})\n",
                Local::now(),
                client_1_addr,
                client_2_addr
            )
            .as_bytes(),
        )?;

        let mut client_1_socket_clone = client_1_socket.try_clone()?;
        let mut client_2_socket_clone = client_2_socket.try_clone()?;

        let client_1_to_client_2 = thread::spawn({
            let score = Arc::clone(&score);
            move || {
                let mut buffer = [0; 12];
                loop {
                    match client_1_socket.read_exact(&mut buffer) {
                        Ok(_) => {
                            println!("player1 sent : {:?}", buffer);
                            if buffer[2] == 0
                                && buffer[3] == 0
                                && (buffer[0] != 0 || buffer[1] != 0)
                            {
                                println!("{:?}", buffer);
                                let mut score = score.lock().unwrap();

                                score[0] = buffer[0] as i32;
                                score[1] = buffer[1] as i32;
                                break;
                            }
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
            }
        });

        let client_2_to_client_1 = thread::spawn({
            let score = Arc::clone(&score);
            move || {
                let mut buffer = [0; 4];

                loop {
                    match client_2_socket_clone.read_exact(&mut buffer) {
                        Ok(_) => {
                            println!("player2 sent : {:?}", buffer);
                            if buffer[2] == 0
                                && buffer[3] == 0
                                && (buffer[0] != 0 || buffer[1] != 0)
                            {
                                println!("u can see the score");
                                let mut score = score.lock().unwrap();
                                score[0] = buffer[0] as i32;
                                score[1] = buffer[1] as i32;
                                println!("{} {}", score[0], score[1]);
                                break;
                            }
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
            }
        });

        client_1_to_client_2.join().unwrap();
        client_2_to_client_1.join().unwrap();
        let mut score_copy = score.lock().unwrap();

        file.write_all(
            format!(
                "game ended at {:?} with player1({}) and player2({})\n",
                Local::now(),
                client_1_addr,
                client_2_addr
            )
            .as_bytes(),
        )?;
        file.write_all(
            format!(
                "total score(player1 : player2) -> {} : {}\n\n",
                score_copy[0], score_copy[1]
            )
            .as_bytes(),
        )?;
    }

    Ok(())
}
