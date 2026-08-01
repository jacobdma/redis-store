use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    // A buffer to hold incoming data
    let mut buffer = [0; 512];

    // Read data from stream and write it back
    while match stream.read(&mut buffer) {
        Ok(size) if size > 0 => {
            stream.write_all(&buffer[0..size]).unwrap();
            true
        }
        _ => false, // Lower than or equal to 0 means disconnect or error
    } {}
}

fn main() {
    // Bind listener to 6380
    let listener = TcpListener::bind("127.0.0.1:6380").expect("Could not bind");
    println!("Server listening on port 6380...");

    // Accept connections and process them sequentially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());

                // Spawn a new thread for each connection to handle them concurrently
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
