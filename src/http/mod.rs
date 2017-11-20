//! Handles the actual HTTP(S) requests to be sent to the Noms server

use hyper;
use hyper::{Request, Response, Method, StatusCode};
use hyper::client::HttpConnector;
use tokio_core::reactor::Handle;
use futures::{Future, Stream, future};
use error::Error;
use hash::Hash;
use std::collections::HashMap;
use chunk::Chunk;
use byteorder::{NetworkEndian, ByteOrder};

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
pub(crate) struct Client {
    client: hyper::Client<HttpConnector>,
    database: String,
    version: String,
}

impl Client {
    pub fn new(database: String, version: String, handle: &Handle) -> Self {
        Self{ database, version: version, client: hyper::Client::new(handle) }
    }

    fn request_for(&self, method: Method, path: &str) -> hyper::Result<Request> {
        let mut req = Request::new(method, format!("http://{}{}", self.database, path).parse()?);
        req.headers_mut().set(XNomsVersion(self.version.clone()));
        Ok(req)
    }

    fn request_with_query(&self, method: Method, path: &'static str, query: &str) -> hyper::Result<Request> {
        self.request_for(method, &format!("{}?{}", path, query))
    }

    pub fn get_root(&self) -> Box<Future<Item = Hash, Error = Error>> {
        let client = self.client.clone();
        Box::new(
            future::result(self.request_for(Method::Get, ROOT_PATH))
                .and_then(move |req| client.request(req))
                .map_err(|err| Error::Hyper(err))
                .and_then(retrieve_body)
                .map(|chunk| Hash::from_string(String::from_utf8(chunk.to_vec()).unwrap()).unwrap())
        )
    }

    pub fn post_get_refs(&self, root: Hash, refs: Vec<Hash>) -> Box<Future<Item = HashMap<Hash, Chunk>, Error = Error>> {
        let client = self.client.clone();
        let mut body = vec![0; 4];
        NetworkEndian::write_u32(&mut body, refs.len() as u32);
        body.extend(refs.iter().flat_map(|r| r.raw_bytes().to_vec()));
        Box::new(
            future::result(self.request_with_query(Method::Post, GET_REFS_PATH, &format!("root={}", root.to_string())))
                .map(|mut req| { req.set_body(body); req })
                .and_then(move |req| client.request(req))
                .map_err(|err| Error::Hyper(err))
                .and_then(retrieve_body)
                .map(Chunk::from_hyper)
                .map(|chunk| {
                    let reader = chunk.reader();
                    let mut values = HashMap::new();
                    while !reader.empty() {
                        let hash = reader.read_hash();
                        let len = reader.read_u32();
                        let bytes = reader.read_raw(len as usize);
                        values.insert(hash, bytes);
                    }
                    values
                })
        )
    }
}

fn retrieve_body(res: Response) -> Box<Future<Item = hyper::Chunk, Error = Error>> {
    match res.status() {
        StatusCode::Ok => Box::new(res.body().concat2().map_err(|err| Error::Hyper(err))),
        status => Box::new(future::result(Err(Error::Http(status)))),
    }
}
