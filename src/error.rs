//! Defines some errors that can be returned

#[derive(Debug)]
pub enum Error {
    Hyper(::hyper::Error),
    Http(::hyper::StatusCode),
    Hash(&'static str),
}

impl From<::hyper::Error> for Error {
    fn from(err: ::hyper::Error) -> Self { Error::Hyper(err) }
}

impl From<::data_encoding::DecodePartial> for Error {
    fn from(err: ::data_encoding::DecodePartial) -> Self { Error::Hash("Could not decode hash") }
}

impl From<::std::string::FromUtf8Error> for Error {
    fn from(err: ::std::string::FromUtf8Error) -> Self { Error::Hash("Hash data was not valid UTF-8") }
}
