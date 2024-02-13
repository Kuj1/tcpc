pub mod handlers;
pub mod menu;
pub mod errors;

use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener};

use errors::{SendResult, RecvError, RecvResult};
use crate::handlers::RequestHandlers;

pub fn send_result<Writer: Write>(data: &str, mut writer: Writer) -> SendResult {
    let bytes = data.as_bytes();
    writer.write_all(bytes)?;
    println!("data: {}", &data);
    Ok(())
}

pub fn recive_command<'a>(mut stream: &'a TcpStream, mut buf: &mut [u8]) -> RecvResult<'a> {
    stream.read(&mut buf)?;
    let i = String::from_utf8(buf.to_vec()).map_err(|_| RecvError::BadEncoding);
    i
}


fn handle_connection(stream: &TcpStream) -> Result<(), errors::SendError> {
    println!("Connected: {}", stream.peer_addr().unwrap());
    let mut handler = RequestHandlers::new();
    loop {
        let mut buf = [0; 4];
        match recive_command(stream, &mut buf) {
            Ok(command) => println!("{:#?}", command),
            Err(e) => eprint!("[ERROR]: {}",e)
        }

        let request = std::str::from_utf8(&buf);
        let resp = handler.handle(request.unwrap());
        let sended = send_result(&resp, stream);
        return sended;
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4343")?;
    for stream in listener.incoming() {
        let mut conn = match stream {
            Ok(c) => c,
            Err(e) => {eprintln!("[Wrong connection]: {}", e); continue;}
        };
        loop {
            handle_connection(&mut conn).expect("[ERROR]: While handle connection");
        }
    }
    Ok(())
}
