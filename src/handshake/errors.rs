use std::io;
use nom;

pub enum HandshakeError {
    NomError(nom::Err<nom::error::ErrorKind>),
    IoError(io::Error),
    VersionError(u8),
    EchoMismatch,
    HandshakeAlreadyDone,
}

impl From<nom::Err<nom::error::ErrorKind>> for HandshakeError {
    fn from(err: nom::Err<nom::error::ErrorKind>) -> Self {
        HandshakeError::NomError(err)
    }
}

impl From<io::Error> for HandshakeError {
    fn from(err: io::Error) -> Self {
        HandshakeError::IoError(err)
    }
}