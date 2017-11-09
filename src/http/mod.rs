//! Handles the actual HTTP(S) requests to be sent to the Noms server

use hyper;
use hyper::{Client, Request, Method, StatusCode};
use hyper::header::Header;
use hyper::client::HttpConnector;
use tokio_core::reactor::Core;
use futures::{Future, Stream};
use std::fmt;
use error::Error;

use chunk::Chunk;

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

pub struct HttpClient {
    client: Client<HttpConnector>,
    database: String,
    version: String,
}

impl HttpClient {
    pub fn new(database: String, version: String) -> Self {
        Self{ database, version: version, client: Client::new(&Core::new().unwrap().handle()) }
    }

    fn request_for(&self, method: Method, path: &'static str) -> hyper::Result<Request> {
        let mut req = Request::new(method, (self.database.clone() + path).parse()?);
        req.headers_mut().set(XNomsVersion(self.version.clone()));
        Ok(req)
    }

    pub fn get_root(&self) -> Result<Chunk, Error> {
        let req = self.request_for(Method::Get, ROOT_PATH)?;
        let res = self.client.request(req).wait()?;
        match res.status() {
            StatusCode::Ok => Ok(Chunk::from_bytes(&*res.body().concat2().wait()?)),
            status => Err(Error::Http(status))
        }
    }
}
