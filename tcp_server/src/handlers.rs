use connector::connector::Connector;

pub struct RequestHandlers {
    connector: Connector,
}

impl RequestHandlers {
    // TODO: move outside the handler
    pub fn new() -> Self {
        Self {
            connector: Connector::new(),
        }
    }

    pub fn set_enabled(&mut self, is_enable: &str) -> String {
        self.connector.on_off(is_enable)
    }

    pub fn on(&mut self) -> String {
        self.set_enabled("on")
    }

    pub fn off(&mut self) -> String {
        self.set_enabled("off")
    }

    pub fn handle(&mut self, request: &str) -> String {
        let command = request.split(":").next().unwrap();
        match command {
            "create" => {
                let req_h = RequestHandlers::new();
                format!("{:#?}", req_h.connector)
            }
            "on" => self.on(),
            "off" => self.off(),
            _ => "Bad command".into(),
        }
    }
}
