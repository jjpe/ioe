/// A crate that makes many of the I/O errors in stdlib de/serializable.

extern crate serde;
#[macro_use] extern crate serde_derive;

use std::error::Error;
use std::fmt;
use std::io;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Serialize, Deserialize)]
pub struct IoError {
    kind: IoErrKind,
    description: String,
}

impl From<io::Error> for IoError {
    fn from(err: io::Error) -> IoError {
        IoError {
            kind: IoErrKind::from(err.kind()),
            description: String::from(err.description()),
        }
    }
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.description)
    }
}


/// A copy of `io::ErrorKind` that can be properly de/serialized.
/// It's possible to convert between io::ErrorKind and `IoErrKind`
/// by using the From trait.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Serialize, Deserialize)]
pub enum IoErrKind {
    NotFound,
    PermissionDenied,
    ConnectionRefused,
    ConnectionReset,
    ConnectionAborted,
    NotConnected,
    AddrInUse,
    AddrNotAvailable,
    BrokenPipe,
    AlreadyExists,
    WouldBlock,
    InvalidInput,
    InvalidData,
    TimedOut,
    WriteZero,
    Interrupted,
    Other,
    UnexpectedEof,
    #[doc(hidden)] __Nonexhaustive,
}

impl fmt::Display for IoErrKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<io::ErrorKind> for IoErrKind {
    fn from(kind: io::ErrorKind) -> IoErrKind {
        match kind {
            io::ErrorKind::NotFound => IoErrKind::NotFound,
            io::ErrorKind::PermissionDenied => IoErrKind::PermissionDenied,
            io::ErrorKind::ConnectionRefused => IoErrKind::ConnectionRefused,
            io::ErrorKind::ConnectionReset => IoErrKind::ConnectionReset,
            io::ErrorKind::ConnectionAborted => IoErrKind::ConnectionAborted,
            io::ErrorKind::NotConnected => IoErrKind::NotConnected,
            io::ErrorKind::AddrInUse => IoErrKind::AddrInUse,
            io::ErrorKind::AddrNotAvailable => IoErrKind::AddrNotAvailable,
            io::ErrorKind::BrokenPipe => IoErrKind::BrokenPipe,
            io::ErrorKind::AlreadyExists => IoErrKind::AlreadyExists,
            io::ErrorKind::WouldBlock => IoErrKind::WouldBlock,
            io::ErrorKind::InvalidInput => IoErrKind::InvalidInput,
            io::ErrorKind::InvalidData => IoErrKind::InvalidData,
            io::ErrorKind::TimedOut => IoErrKind::TimedOut,
            io::ErrorKind::WriteZero => IoErrKind::WriteZero,
            io::ErrorKind::Interrupted => IoErrKind::Interrupted,
            io::ErrorKind::Other => IoErrKind::Other,
            io::ErrorKind::UnexpectedEof => IoErrKind::UnexpectedEof,
            kind => panic!("Unknown enum variant io::ErrorKind::{:?}", kind),
        }
    }
}

impl From<IoErrKind> for io::ErrorKind {
    fn from(kind: IoErrKind) -> io::ErrorKind {
        match kind {
            IoErrKind::NotFound => io::ErrorKind::NotFound,
            IoErrKind::PermissionDenied => io::ErrorKind::PermissionDenied,
            IoErrKind::ConnectionRefused => io::ErrorKind::ConnectionRefused,
            IoErrKind::ConnectionReset => io::ErrorKind::ConnectionReset,
            IoErrKind::ConnectionAborted => io::ErrorKind::ConnectionAborted,
            IoErrKind::NotConnected => io::ErrorKind::NotConnected,
            IoErrKind::AddrInUse => io::ErrorKind::AddrInUse,
            IoErrKind::AddrNotAvailable => io::ErrorKind::AddrNotAvailable,
            IoErrKind::BrokenPipe => io::ErrorKind::BrokenPipe,
            IoErrKind::AlreadyExists => io::ErrorKind::AlreadyExists,
            IoErrKind::WouldBlock => io::ErrorKind::WouldBlock,
            IoErrKind::InvalidInput => io::ErrorKind::InvalidInput,
            IoErrKind::InvalidData => io::ErrorKind::InvalidData,
            IoErrKind::TimedOut => io::ErrorKind::TimedOut,
            IoErrKind::WriteZero => io::ErrorKind::WriteZero,
            IoErrKind::Interrupted => io::ErrorKind::Interrupted,
            IoErrKind::Other => io::ErrorKind::Other,
            IoErrKind::UnexpectedEof => io::ErrorKind::UnexpectedEof,
            kind => panic!("Unknown enum variant IoErrKind::{:?}", kind),
        }
    }
}
