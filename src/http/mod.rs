//! Handles the actual HTTP(S) requests to be sent to the Noms server

use hyper;
use hyper::{Request, Method, StatusCode};
use hyper::client::HttpConnector;
use tokio_core::reactor::Handle;
use futures::{Future, Stream, future};
use error::Error;
use hash::Hash;
use hash;

const ROOT_PATH: &'static str           = "/root/";
const GET_REFS_PATH: &'static str       = "/getRefs/";
const GET_BLOB_PATH: &'static str       = "/getBlob/";
const HAS_REFS_PATH: &'static str       = "/hasRefs/";
const WRITE_VALUE_PATH: &'static str    = "/writeValue/";
const BASE_PATH: &'static str           = "/";
const GRAPHQL_PATH: &'static str        = "/graphql/";
const STATS_PATH: &'static str          = "/stats/";

const NOMS_VERSION_HEADER: &'static str = "X-Noms-Vers";

header! { (XNomsVersion, NOMS_VERSION_HEADER) => [String] }

#[derive(Clone)]
pub struct Client {
    client: hyper::Client<HttpConnector>,
    database: String,
    version: String,
}

impl Client {
    pub fn new(database: String, version: String, handle: &Handle) -> Self {
        Self{ database, version: version, client: hyper::Client::new(handle) }
    }

    fn request_for(&self, method: Method, path: &'static str) -> hyper::Result<Request> {
        let mut req = Request::new(method, format!("http://{}{}", self.database, path).parse()?);
        req.headers_mut().set(XNomsVersion(self.version.clone()));
        Ok(req)
    }

    pub fn get_root(&self) -> Box<Future<Item = Hash, Error = Error>> {
        let client = self.client.clone();
        Box::new(
            future::result(self.request_for(Method::Get, ROOT_PATH))
                .and_then(move |req| client.request(req))
                .map_err(|err| Error::Hyper(err))
                .and_then(|res| -> Box<Future<Item = _, Error = _>> {
                    match res.status() {
                        StatusCode::Ok => Box::new(res.body().concat2().map_err(|err| Error::Hyper(err))),
                        status => Box::new(future::result(Err(Error::Http(status)))),
                    }
                })
                .and_then(|chunk| hash::parse(&*chunk))
        )
    }
}
