pub mod errors;
pub mod menu;

use std::io::{Read, Write};
use std::net::TcpStream;

use crate::errors::{RecvError, RecvResult, SendError, SendResult};
use menu::MainMenu;

pub fn send_command<Writer: Write>(data: &str, mut writer: Writer) -> SendResult {
    let bytes = data.as_bytes();
    // println!("data bytes: {:?}", bytes);
    // println!("bytes len in u32: {}", bytes.len());
    let len = bytes.len() as u32;
    let len_bytes = len.to_be_bytes();
    // println!("bytes len in be_bytes array: {:?}", len_bytes);
    writer.write_all(&len_bytes)?;
    writer.write_all(bytes)?;

    Ok(())
}

pub fn recieve_result(mut stream: &TcpStream) -> RecvResult {
    let mut buf = [0; 4];
    stream.read_exact(&mut buf)?;
    let len = u32::from_be_bytes(buf);
    let mut buf = vec![0; len as _];
    stream.read_exact(&mut buf)?;
    String::from_utf8(buf).map_err(|_| RecvError::BadEncoding)
}

pub fn shutdown(stream: &TcpStream) -> Result<(), SendError> {
    stream.shutdown(std::net::Shutdown::Both).expect("ERROR");
    Ok(())
}

fn main() -> Result<(), errors::SendError> {
    match TcpStream::connect("127.0.0.1:4343") {
        Ok(stream) => {
            println!("\nConnected to the server!");
            loop {
                MainMenu::choices(&stream).unwrap();
                match recieve_result(&stream) {
                        Ok(result) => println!("\n[START MESSAGE]\n-----------------\n{}\n-----------------\n[END MESSAGE]", result),
                        Err(e) => eprintln!("[ERROR]: {}", e)
                    }
            }
        }
        Err(e) => println!("{}", e),
    };
    Ok(())
}
