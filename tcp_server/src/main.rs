pub mod errors;
pub mod handlers;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::handlers::RequestHandlers;
use connector::connector::Connector;
use errors::{RecvError, RecvResult, SendResult};

pub fn send_result<Writer: Write>(data: &str, mut writer: Writer) -> SendResult {
    let bytes = data.as_bytes();
    let len = bytes.len() as u32;
    let len_bytes = len.to_be_bytes();
    writer.write_all(&len_bytes)?;
    writer.write_all(bytes)?;
    // println!("data: {}", &data);
    Ok(())
}

pub fn recive_command(mut stream: &TcpStream) -> RecvResult {
    let mut buf = [0; 4];
    stream.read_exact(&mut buf)?;
    // println!("{:?}", buf);
    let len = u32::from_be_bytes(buf);
    let mut buf = vec![0; len as _];
    stream.read_exact(&mut buf)?;
    // println!("{:?}", buf);
    String::from_utf8(buf).map_err(|_| RecvError::BadEncoding)
}

fn handle_connection(
    stream: &TcpStream,
    mut rh: RequestHandlers,
    cn: &mut Connector,
) -> Result<(), errors::SendError> {
    let request = match recive_command(stream) {
        Ok(command) => {
            println!("{:#?}", command);
            command
        }
        Err(e) => {
            eprint!("[ERROR]: {}", e);
            format!("[ERROR]: {}", e)
        }
    };

    let resp = rh.handle(&request, cn);
    send_result(&resp, stream)
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4343")?;
    let mut connector = Connector::default();
    if let Some(stream) = listener.incoming().next() {
        println!(
            "Connected: {}",
            stream.as_ref().unwrap().peer_addr().unwrap()
        );
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
