pub mod errors;
pub mod menu;

use std::io::{Read, Write};
use std::net::TcpStream;

use menu::MainMenu;
// use connector::connector::Connector;
use crate::errors::{RecvError, SendResult, RecvResult, SendError};

pub fn send_command<Writer: Write>(data: &str, mut writer: Writer) -> SendResult {
    let bytes = data.as_bytes();
    // println!("bytes len: {}", bytes.len());
    writer.write_all(bytes)?;
    // println!("data: {}", &data);
    Ok(())
}

// TODO: recive result from server
pub fn receive_result(mut stream: &TcpStream) -> RecvResult {
    let mut buf = [0; 50];
    stream.read(&mut buf)?;
    let i = String::from_utf8(buf.to_vec()).map_err(|_| RecvError::BadEncoding);
    i
}

pub fn shutdown(stream: &TcpStream) -> Result<(), SendError>  {
    stream.shutdown(std::net::Shutdown::Both).expect("ERROR");
    Ok(())
}

fn main() -> Result<(), errors::SendError> {
    match TcpStream::connect("127.0.0.1:4343") {
            Ok(stream) => {
                println!("\nConnected to the server!");
                loop {
                    MainMenu::choices(&stream).unwrap(); 
                    match receive_result(&stream) {
                        Ok(result) => println!("\n[START MESSAGE]\n-----------------\n{}\n-----------------\n[END MESSAGE]", result),
                        Err(e) => eprintln!("[ERROR]: {}", e)
                    }
                }
            },
            Err(e) => println!("{}", e)
        };
    Ok(())
}