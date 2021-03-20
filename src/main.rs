mod client;
mod server;

fn main() -> std::io::Result<()> {
    let addr = String::from("127.0.0.1");
    let port = String::from("1337");

    let mut args = std::env::args().skip(1);
    let command = args.next().expect("no command found");
    match command.as_ref() {
        "server" => {
            let listener = server::new(addr, port).unwrap();
            return server::start_listen(listener);
        }
        "client" => {
            let conn_string = client::format_conn_string(addr, port);
            client::connect_to_server(conn_string);
        }
        _ => return Ok(()),
    };
    Ok(())
}
