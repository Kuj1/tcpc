pub mod errors;
pub mod menu;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::errors::{RecvError, RecvResult, SendError, SendResult};
use menu::MainMenu;

pub async fn send_command(data: &str, writer: &mut TcpStream) -> SendResult {
    let bytes = data.as_bytes();
    println!("data bytes: {:?}", bytes);
    println!("bytes len in u32: {}", bytes.len());
    let len = bytes.len() as u32;
    let len_bytes = len.to_be_bytes();
    println!("bytes len in be_bytes array: {:?}", len_bytes);
    writer.write_all(&len_bytes).await?;
    writer.write_all(bytes).await?;
    Ok(())
}

pub async fn recieve_result(stream: &mut TcpStream) -> RecvResult {
    let mut buf = [0; 4];
    stream.read_exact(&mut buf).await?;
    let len = u32::from_be_bytes(buf);
    let mut buf = vec![0; len as _];
    stream.read_exact(&mut buf).await?;
    String::from_utf8(buf).map_err(|_| RecvError::BadEncoding)
}

pub async fn shutdown(stream: &mut TcpStream) -> Result<(), SendError> {
    stream.shutdown().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), errors::SendError> {
    match TcpStream::connect("127.0.0.1:4343").await {
        Ok(mut stream) => {
            println!("\nConnected to the server!");
            loop {
                MainMenu::choices(&mut stream).await?;
                match recieve_result(&mut stream).await {
                        Ok(result) => println!("\n[START MESSAGE]\n-----------------\n{}\n-----------------\n[END MESSAGE]", result),
                        Err(e) => eprintln!("[ERROR]: {}", e)
                    }
            }
        }
        Err(e) => println!("{}", e),
    };
    Ok(())
}
