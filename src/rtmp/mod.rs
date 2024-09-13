pub mod packets;
mod reader;
mod writer;

use crate::context::{NetConnectionContext, ConnectionArgs};

#[derive(Debug)]
struct NetConnection {
    context: NetConnectionContext
}

// impl NetConnection {
//     pub fn new(connection_args: ConnectionArgs) {
//     }

//     pub fn on_connect() {
//     }

    
// }

