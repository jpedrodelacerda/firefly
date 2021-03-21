use std::io;
use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};

use firefly::Codec;

fn format_conn_string(addr: String, port: String) -> String {
    format!("{}:{}", addr, port)
}

fn connect_to_server(conn_string: String) -> io::Result<()> {
    let stream = TcpStream::connect(conn_string).unwrap();
    let mut codec = Codec::new(stream)?;
	println!("{}", codec.read_message()?);
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(msg) => {
                let msg_size = codec.send_message(&msg)?;
				if msg_size == 0 {
					break
				}
            }
            Err(_) => {
				println!("failed to send message");
                codec.send_message("failed to write")?;
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let addr = String::from("127.0.0.1");
    let port = String::from("1337");
    let conn_string = format_conn_string(addr, port);
    connect_to_server(conn_string)
}
