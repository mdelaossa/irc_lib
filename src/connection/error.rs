use derive_more::From;

use crate::message::Error as MessageError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // -- Externals
    #[from]
    #[allow(dead_code)]
    Io(std::io::Error),
    #[from]
    MessageParsingError(MessageError),
    NotConnected,
    ConnectionClosed,
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
