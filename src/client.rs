use std::io;
use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};

pub fn format_conn_string(addr: String, port: String) -> String {
    format!("{}:{}", addr, port)
}

pub fn connect_to_server(conn_string: String) -> io::Result<()> {
    let mut client = TcpStream::connect(conn_string).unwrap();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(msg) => {
                let msg_size = client.write(&msg.into_bytes())?;
                if msg_size == 0 {
                    client.shutdown(Shutdown::Both).expect("failed to shutdown");
                    break;
                }
                match client.flush() {
                    Ok(()) => {
                        println!("flushed successfully");
                    }
                    Err(_e) => {
                        println!("flushed failed");
                        client.shutdown(Shutdown::Both).expect("shutdown failed")
                    }
                }
            }
            Err(_) => {
                client.write(b"failed to write")?;
            }
        }
    }
	Ok(())
}
