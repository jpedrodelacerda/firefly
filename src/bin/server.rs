use std::io;
use std::net::TcpListener;

use firefly::Codec;

fn start_listen(listener: TcpListener) -> io::Result<()> {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("new client");
                let codec = Codec::new(stream)?;

                handle_client(codec);
            }
            Err(e) => {
                println!("failed to read:\n{}", e);
            }
        }
    }
    Ok(())
}

fn handle_client(mut codec: Codec) {
    codec.send_message("welcome to firefly!\n").unwrap();
    loop {
        let line = codec.read_message();
        match line {
            Ok(msg) => println!("{}", msg),
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
    }
}

fn new(address: String, port: String) -> io::Result<TcpListener> {
    let conn_string = format!("{}:{}", address, port);
    return TcpListener::bind(conn_string);
}

fn main() -> io::Result<()> {
    let addr = String::from("127.0.0.1");
    let port = String::from("1337");
    let listener = new(addr, port).unwrap();
    return start_listen(listener);
}
