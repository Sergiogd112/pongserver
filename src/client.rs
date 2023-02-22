use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");
            let msg_txt = "Hello:";
            let msg = msg_txt.as_bytes();
            let text = from_utf8(msg).unwrap();

            stream.write(msg).unwrap();
            println!("Sent {}, awaiting reply...", text);

            let mut data = [0 as u8; 2]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == b"OK" {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
            let msg_txt = "Hello:";
            let msg = msg_txt.as_bytes();
            let text = from_utf8(msg).unwrap();

            stream.write(msg).unwrap();
            println!("Sent {}, awaiting reply...", text);

            let mut data = [0 as u8; 2]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == b"OK" {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
            let msg = b"close";
            stream.write(msg).unwrap();
            println!("Sent close, awaiting reply...");

            let mut data = [0 as u8; 2]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == b"OK" {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
            
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
