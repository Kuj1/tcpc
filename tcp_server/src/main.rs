pub mod errors;
pub mod handlers;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::handlers::RequestHandlers;
use connector::connector::Connector;
use errors::{RecvError, RecvResult, SendResult};

pub fn send_result<Writer: Write>(data: &str, mut writer: Writer) -> SendResult {
    let bytes = data.as_bytes();
    writer.write_all(bytes)?;
    println!("data: {}", &data);
    Ok(())
}

pub fn recive_command<'a>(mut stream: &'a TcpStream, mut buf: &mut [u8]) -> RecvResult<'a> {
    stream.read_exact(&mut buf)?;
    String::from_utf8(buf.to_vec()).map_err(|_| RecvError::BadEncoding)
}

fn handle_connection(stream: &TcpStream, mut rh: RequestHandlers, cn: &mut Connector) -> Result<(), errors::SendError> {
    println!("Connected: {}", stream.peer_addr().unwrap());
    loop {
        let mut buf = [0; 6];
        match recive_command(stream, &mut buf) {
            Ok(command) => println!("{:#?}", command),
            Err(e) => eprint!("[ERROR]: {}", e),
        }

        let request = std::str::from_utf8(&buf);
        let resp = rh.handle(request.unwrap(), cn);
        let sended = send_result(&resp, stream);
        return sended;
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4343")?;
    // let handler = RequestHandlers;
    let mut connector = Connector::default();
    for stream in listener.incoming() {
        // let mut handler = RequestHandlers;
        // let mut connector = handler.create();
        loop {
            let handler = RequestHandlers;
            let connector = &mut connector;
            let conn = match stream {
                Ok(ref c) => c,
                Err(ref e) => {
                    eprintln!("[Wrong connection]: {}", e);
                    continue;
                }
            };
            handle_connection(conn, handler, connector).expect("[ERROR]: While handle connection");
        }
    }
    Ok(())
}
