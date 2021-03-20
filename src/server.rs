use std::io;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

pub fn start_listen(listener: TcpListener) -> io::Result<()> {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("new client");
                handle_client(stream);
            }
            Err(e) => {
                println!("failed to read:\n{}", e);
            }
        }
    }
    Ok(())
}

pub fn handle_client(mut stream: TcpStream) {
    let mut reader = io::BufReader::new(&mut stream);

    loop {
        let received = reader.fill_buf().unwrap().to_vec();
        println!("received: {} bytes", received.len());
        if received.len() == 0 {
            println!("Connection closed.");
            break;
        }
        reader.consume(received.len());

        let _str = String::from_utf8(received)
            .map(|msg| println!("{}", msg))
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse string"));
    }
}

pub fn new(address: String, port: String) -> io::Result<TcpListener> {
    let conn_string = format!("{}:{}", address, port);
    return TcpListener::bind(conn_string);
}
