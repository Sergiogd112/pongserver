use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread::{self, JoinHandle};

fn handle_client(mut stream: TcpStream) -> bool {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    let mut pdata = data;
    let mut n = 0;
    while match stream.read(&mut data) {
        Ok(_) => {
            // echo everything!
            stream.write(b"OK").unwrap();
            println!("replying");
            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {
        if pdata != data {
            let text: &str = from_utf8(&data).unwrap();
            println!("{},{}", text, n);
            n = 0;
        } else {
            n += 1;
        }
        pdata = data;
        match &from_utf8(&data).unwrap()[0..5] {
            "close" => {
                println!("closing thread");
                break;
            }
            "start" => {}
            "joinp" => {}
            "termiate" => {
                return true;
            }
            _ => {}
        };
    }
    return false;
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    let handles:Vec<JoinHandle<bool>>=Vec::new();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let mut data = [0 as u8; 50]; // using 50 byte buffer
                match stream.read(&mut data) {
                    Ok(_) => {
                        // echo everything!
                        stream.write(b"OK").unwrap();
                    }
                    Err(_) => {
                        println!(
                            "An error occurred, terminating connection with {}",
                            stream.peer_addr().unwrap()
                        );
                        stream.shutdown(Shutdown::Both).unwrap();
                    }
                };
                let text: &str = from_utf8(&data).unwrap();
                println!("{}", text);
                let handle = thread::spawn(move || {
                    // connection succeeded
                    let out:bool = handle_client(stream);
                    println!("ended");
                    out
                });
                handles.push(&mut handle);
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
