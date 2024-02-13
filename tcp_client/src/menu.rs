use std::{io, net::TcpStream};

use crate::{errors::SendError, send_command, shutdown};

// #[derive(PartialEq)]
// pub enum Options {
//     CreateConnector,
//     GetStatus,
//     On,
//     Off,
//     Exit,
// }

pub struct MainMenu;

impl MainMenu {
    pub fn choices(stream: &TcpStream) -> Result<MainMenu, SendError>{
        println!("
            Choose one option:\n
                \t1) Create Connector
                \t2) On Connector;
                \t3) Off Connector;
                \t4) Get Status;
                \tOther) Exit.
        ");

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("Not an option");

        let selected: &str = buf.trim();
        println!("Selected: {}", selected);

        let _ = match selected {
            "1" => send_command("on::", stream),
            "4" => send_command("stat", stream),
            "2" => send_command("on::", stream),
            "3" => send_command("off:", stream),
            _ => shutdown(stream)
        };

        Ok(MainMenu)
    }
}

pub struct ConnectorMenu;

impl ConnectorMenu {}