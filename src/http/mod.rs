//! Handles the actual HTTP(S) requests to be sent to the Noms server

use hyper;
use hyper::{Request, Method, StatusCode};
use hyper::client::HttpConnector;
use tokio_core::reactor::Handle;
use futures::{Future, Stream};
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

    pub fn get_root(&self) -> Result<Hash, Error> {
        let req = self.request_for(Method::Get, ROOT_PATH)?;
        println!("{:?}", req);
        let res = self.client.request(req).wait()?;
        println!("{:?}", res);
        match res.status() {
            StatusCode::Ok => hash::parse(String::from_utf8((*res.body().concat2().wait()?).to_vec())?),
            status => Err(Error::Http(status))
        }
    }
}
