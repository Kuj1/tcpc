use std::{io, net::TcpStream};

use crate::{errors::SendError, send_command, shutdown};

pub struct MainMenu;

impl MainMenu {
    pub fn choices(stream: &TcpStream) -> Result<MainMenu, SendError> {
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
        println!("Selected: {}", selected);

        let _ = match selected {
            "1" => send_command("on::::", stream),
            "2" => send_command("off:::", stream),
            "3" => send_command("stat::", stream),
            _ => shutdown(stream),
        };

        Ok(MainMenu)
    }
}
