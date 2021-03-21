use std::io::Error;
use std::io::{self, BufRead, Write};
use std::net::TcpStream;

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
