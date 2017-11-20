//! Defines some errors that can be returned
use hash::Hash;

#[derive(Debug)]
pub enum Error {
    Hyper(::hyper::Error),
    Http(::hyper::StatusCode),
    Hash(String),
    NoDataset(String),
    NoValueForRef(Hash),
}

impl From<::hyper::Error> for Error {
    fn from(err: ::hyper::Error) -> Self { Error::Hyper(err) }
}

impl From<::data_encoding::DecodePartial> for Error {
    fn from(err: ::data_encoding::DecodePartial) -> Self { Error::Hash(format!("Could not decode hash: {:?}", err)) }
}

impl From<::std::string::FromUtf8Error> for Error {
    fn from(err: ::std::string::FromUtf8Error) -> Self { Error::Hash(format!("Hash data was not valid UTF-8: {:?}", err)) }
}
