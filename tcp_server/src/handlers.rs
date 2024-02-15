use connector::connector::Connector;

pub struct RequestHandlers;

impl RequestHandlers {
    pub fn on(&mut self, connector: &mut Connector) -> String {
        connector.on_off("on")
    }

    pub fn off(&mut self, connector: &mut Connector) -> String {
        connector.on_off("off")
    }

    pub fn get_status(&mut self, connector: &Connector) -> String {
        format!("{:#?}", connector)
    }

    pub fn handle(&mut self, request: &str, connector: &mut Connector) -> String {
        match request {
            "on" => self.on(connector),
            "off" => self.off(connector),
            "stat" => self.get_status(connector),
            _ => "Bad command".into(),
        }
    }
}
