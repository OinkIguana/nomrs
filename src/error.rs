//! Defines some errors that can be returned

pub enum Error {
    Hyper(::hyper::Error),
    Http(::hyper::StatusCode),
}

impl From<::hyper::Error> for Error {
    fn from(err: ::hyper::Error) -> Self { Error::Hyper(err) }
}
