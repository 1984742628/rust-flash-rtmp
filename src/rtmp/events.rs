#[derive(Debug)]
pub enum StatusChange {
    Connected,
    Disconnected,
}

#[derive(Debug)]
pub enum TransportEvent {
    StatusChange(StatusChange),
    MessageReceived(Vec<u8>),
    ErrorOccurred(String),
}