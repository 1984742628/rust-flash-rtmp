mod events;
mod reader;
mod writer;

use crate::handshake::{HandshakeResult, RTMPHandshakeNegiotator};
use crate::rtmp::events::{TransportEvent, StatusChange};

#[derive(Debug)]
struct RTMPConnectionArgs {
    tc_url: String,
    page_url: String,
    swf_url: String,
    app: String
}

#[derive(Debug)]
struct RTMPClient {
    negiotator: Option<RTMPHandshakeNegiotator>,
    connection_args: RTMPConnectionArgs,
}

impl RTMPClient {
    pub fn new(connection_args: RTMPConnectionArgs) -> RTMPClient {
        RTMPClient {
            negiotator: Some(RTMPHandshakeNegiotator::new()),
            connection_args
        }
    }

    pub fn on_status_change(&mut self, change: StatusChange) -> Option<Vec<u8>> {
        match change {
            StatusChange::Connected => {
                // should be only fired once, so it's safe to assume that negiotator isn't None
                let negiotator = self.negiotator.as_mut().unwrap();
                let consumed_result = negiotator.consume(None);

                match consumed_result {
                    Ok(result) => {
                        match result {
                            HandshakeResult::InProgress { response } => Some(response),
                            HandshakeResult::Done { response } => panic!("This shouldn't happen...!")
                        }
                    },
                    Err(err) => panic!("Found error on handshake"),
                }
            },
            _ => { panic!("Unhandled") }
        }
    }

    pub fn on_message(&mut self, data: Vec<u8>) -> Option<Vec<u8>> {
        None
    }

    pub fn handle_transport_event(&mut self, event: TransportEvent) -> Option<Vec<u8>> {
        match event {
            TransportEvent::StatusChange(change) => self.on_status_change(change),
            TransportEvent::MessageReceived(data) => self.on_message(data),
            TransportEvent::ErrorOccurred(error) => panic!("{:?}", error),
        }
    }
}