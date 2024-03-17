pub trait Broker {
    fn name(self) -> String;
}

pub struct NoBroker {}

impl Broker for NoBroker {
    fn name(self) -> String {
        String::from("No Broker")
    }
}
