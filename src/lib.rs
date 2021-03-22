use std::collections::HashMap;
use std::io::Error;
use std::io::{self, BufRead, Write};
use std::net::{TcpListener, TcpStream};

pub struct Codec {
    // stream: TcpStream,
    reader: io::BufReader<TcpStream>,
    writer: io::BufWriter<TcpStream>,
}

impl Codec {
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        let writer = io::BufWriter::new(stream.try_clone().expect("failed to clone stream"));
        let reader = io::BufReader::new(stream);
        Ok(Self { reader, writer })
    }

    pub fn send_message(&mut self, message: &str) -> io::Result<usize> {
        let msg_size = self.writer.write(&message.as_bytes())?;
        self.writer.flush()?;
        Ok(msg_size)
    }

    pub fn read_message(&mut self) -> Result<String, Error> {
        let received = self.reader.fill_buf()?.to_vec();
        if received.len() == 0 {
            return Err(io::Error::new(
                io::ErrorKind::ConnectionAborted,
                "Connection closed.",
            ));
        }
        self.reader.consume(received.len());
        let msg = String::from_utf8(received).unwrap();
        Ok(msg)
    }
}

pub struct Server {
    chats: HashMap<String, Chat>,
    listener: TcpListener,
}

impl Server {
    pub fn new(conn_string: String) -> io::Result<Server> {
        let listener = TcpListener::bind(conn_string).unwrap();
        let chats: HashMap<String, Chat> = HashMap::new();
        Ok(Self {
            chats,
            listener: listener,
        })
    }

    pub fn create_chat(mut self, key: String, conn1: Codec) -> io::Result<()> {
        let chat_key = &key.clone();
        let chat = Chat::new(key, conn1).unwrap();
        match self.chats.get(chat_key) {
            Some(k) => Err(Error::new(
                io::ErrorKind::AlreadyExists,
                format!("the key {} is already in use", k.key),
            )),
            None => {
                self.chats.insert(chat_key.to_string(), chat);
                match self.chats.get(chat_key) {
                    Some(_c) => Ok(()),
                    None => Err(Error::new(
                        io::ErrorKind::PermissionDenied,
                        "failed to create chat",
                    )),
                }
            }
        }
    }

    pub fn start_listen(self) -> io::Result<()> {
        println!("Starting firefly server on {}", self.listener.local_addr()?);
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("new client");
                    let mut conn = Codec::new(stream)?;

                    conn.send_message("welcome to firefly!\n").unwrap();
                    loop {
                        let line = conn.read_message();
                        match line {
                            Ok(msg) => println!("{}", msg),
                            Err(e) => {
                                println!("{}", e);
                                break;
                            }
                        }
                    }

                    // self.create_chat(String::from("kkkk"), conn);
                }
                Err(e) => {
                    println!("failed to read:\n{}", e);
                }
            }
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct Chat {
    pub key: String,
    conn1: Option<Codec>,
    conn2: Option<Codec>,
}

impl Chat {
    pub fn new(key: String, conn1: Codec) -> io::Result<Self> {
        Ok(Self {
            key,
            conn1: Some(conn1),
            conn2: None,
        })
    }

    pub fn connect_to_chat(mut self, conn2: Codec) {
        self.conn2 = Some(conn2)
    }
}
