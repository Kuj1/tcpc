use connector::connector::Connector;

#[derive(Clone)]
pub struct Menu {}

impl Menu {
    pub fn create(&self) -> Connector {
        Connector::new()
    }

    pub fn on(&self, connector: &mut Connector) -> String {
        connector.on_off("on")
    }

    pub fn off(&self, connector: &mut Connector) -> String {
        connector.on_off("off")
    }

    pub fn status(&self, connector: &Connector) -> String {
        connector.get_data()
    }
}