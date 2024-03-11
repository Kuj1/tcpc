use std::io;

use tokio::net::TcpStream;

use crate::{errors::SendError, send_command, shutdown};

pub struct MainMenu;

impl MainMenu {
    pub async fn choices(stream: &mut TcpStream) -> Result<MainMenu, SendError> {
        println!(
            "
            Choose one option:\n
                \t1) On Connector;
                \t2) Off Connector;
                \t3) Get Status;
                \tOther) Exit.
        "
        );

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("Not an option");

        let selected: &str = buf.trim();

        let _ = match selected {
            "1" => send_command("on", stream).await,
            "2" => send_command("off", stream).await,
            "3" => send_command("stat", stream).await,
            _ => shutdown(stream).await,
        };

        Ok(MainMenu)
    }
}
