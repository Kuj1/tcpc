pub mod errors;
pub mod handlers;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::handlers::RequestHandlers;
use connector::connector::Connector;
use errors::{RecvError, RecvResult, SendResult};

pub async fn send_result(data: &str, writer: &mut TcpStream) -> SendResult {
    let bytes = data.as_bytes();
    let len = bytes.len() as u32;
    let len_bytes = len.to_be_bytes();
    writer.write_all(&len_bytes).await?;
    writer.write_all(bytes).await?;
    Ok(())
}

pub async fn recive_command(stream: &mut TcpStream) -> RecvResult {
    let mut buf = [0; 4];
    stream.read_exact(&mut buf).await?;
    println!("bytes len in be_bytes array: {:?}", buf);
    let len = u32::from_be_bytes(buf);
    println!("len in u32: {}", len);
    let mut buf = vec![0; len as _];
    println!("vec for new buff of len 32: {:?}", buf);
    stream.read_exact(&mut buf).await?;
    println!("read buff and see our data bytes: {:?}", buf);
    String::from_utf8(buf).map_err(|_| RecvError::BadEncoding)
}

async fn handle_connection(
    stream: &mut TcpStream,
    mut rh: RequestHandlers,
    cn: &mut Connector,
) -> Result<(), errors::SendError> {
    let request = match recive_command(stream).await {
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
    send_result(&resp, stream).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:4343").await?;
    let mut connector = Connector::default();
    loop {
        let (mut stream, _) = listener.accept().await?;
        println!("Connected: {}", stream.peer_addr().unwrap());
        // let (mut stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            loop {
                let handler = RequestHandlers;
                let connector = &mut connector;
                handle_connection(&mut stream, handler, connector)
                    .await
                    .unwrap_or_else(|_| {
                        panic!(
                            "[ERROR]: While handle connection. {} are disconnected!",
                            stream.peer_addr().unwrap()
                        )
                    });
            }
        });
    }
}
