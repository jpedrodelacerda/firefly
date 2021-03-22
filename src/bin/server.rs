use std::io;

use firefly::Server;

fn main() -> io::Result<()> {
    let addr = String::from("127.0.0.1");
    let port = String::from("1337");
	let conn_string = format!("{}:{}", addr, port);
	let server = Server::new(conn_string).unwrap();
	server.start_listen()
}
