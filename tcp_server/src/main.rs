pub mod errors;
pub mod handlers;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::handlers::RequestHandlers;
use connector::connector::Connector;
use errors::{RecvError, RecvResult, SendResult};

pub fn send_result<Writer: Write>(data: &str, mut writer: Writer) -> SendResult {
    let bytes = data.as_bytes();
    let len = bytes.len() as u32;
    let len_bytes = len.to_be_bytes();
    writer.write_all(&len_bytes)?;
    writer.write_all(bytes)?;
    Ok(())
}

pub fn recive_command(mut stream: &TcpStream) -> RecvResult {
    let mut buf = [0; 4];
    stream.read_exact(&mut buf)?;
    // println!("bytes len in be_bytes array: {:?}", buf);
    let len = u32::from_be_bytes(buf);
    // println!("len in u32: {}", len);
    let mut buf = vec![0; len as _];
    // println!("vec for new buff of len 32: {:?}", buf);
    stream.read_exact(&mut buf)?;
    // println!("read buff and see our data bytes: {:?}", buf);
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
    for stream in listener.incoming() {
        println!(
            "Connected: {}",
            stream.as_ref().unwrap().peer_addr().unwrap()
        );

        thread::spawn(move || loop {
            let handler = RequestHandlers;
            let connector = &mut connector;
            let conn = match stream {
                Ok(ref c) => c,
                Err(ref e) => {
                    eprintln!("[Wrong connection]: {}", e);
                    continue;
                }
            };
            handle_connection(conn, handler, connector).unwrap_or_else(|_| {
                panic!(
                    "[ERROR]: While handle connection. {} are disconnected!",
                    stream.as_ref().unwrap().peer_addr().unwrap()
                )
            });
        });
    }

    Ok(())
}
