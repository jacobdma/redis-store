use std::{
    io::{BufReader, BufRead, Read, Write, Error},
    net::{TcpListener, TcpStream},
    thread,
};

fn handle_client(stream: TcpStream) -> Result<(), Error> {
    // A buffer reader
    let mut reader = BufReader::new(&stream);
    let mut buf = String::new();
    let mut s = &stream;

    // Read data from stream and write it back
    while match reader.read_line(&mut buf) {
        Ok(size) if size > 0 => {
            // Trim trailing whitespace
            let b = buf.trim_end();

            // Check for command
            if let Some(n) = b.strip_prefix("*").and_then(|t| t.parse().ok()) {
                let mut vec: Vec<Vec<u8>> = Vec::new();

                // Loop through buf
                for _ in 0..n {
                    buf.clear();
                    reader.read_line(&mut buf)?;
                    if let Some(len) = buf.trim_end().strip_prefix("$").and_then(|t| t.parse().ok()) {
                        buf.clear();
                        let mut buffer = vec![0; len + 2];
                        reader.read_exact(&mut buffer)?;
                        buffer.truncate(len);
                        vec.push(buffer);
                    }
                }
                println!("{:?}", vec);

                // Command processing
                if vec[0] == b"PING" {
                    s.write_all(b"+PONG\r\n")?;
                } else if vec[0] == b"ECHO" {
                    let x = &vec[1];
                    write!(s, "${}\r\n", x.len())?;
                    s.write_all(x)?;
                    s.write_all(b"\r\n")?;
                }
            }
            true
        }
        _ => false, // Lower than or equal to 0 means disconnect or error
    } { buf.clear(); }
    Ok(())
}

fn main() {
    // Bind listener to 6380
    let listener = TcpListener::bind("127.0.0.1:6380").unwrap();
    println!("Server listening on port 6380...");

    // Accept connections and process them sequentially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());

                // Spawn a new thread for each connection to handle them concurrently
                thread::spawn(|| {
                    if let Err(e) = handle_client(stream){
                        eprintln!("{e}");
                    }
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
