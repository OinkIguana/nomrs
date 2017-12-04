//! Handles the actual HTTP(S) requests to be sent to the Noms server

use hyper;
use hyper::{Request, Response, Method, StatusCode};
use hyper::client::HttpConnector;
use tokio_core::reactor::Handle;
use futures::{Future, Stream, future};
use error::Error;
use hash::{Hash, BYTE_LEN};
use std::collections::{HashSet, HashMap};
use byteorder::{NetworkEndian, ByteOrder};
use database::ChunkStore;

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
                .map(|chunk| Hash::from_string(&String::from_utf8(chunk.to_vec()).unwrap()).unwrap())
        )
    }

    pub fn post_get_refs<'a>(&self, database: &'a ChunkStore, refs: HashSet<Hash>) -> Box<Future<Item = HashMap<Hash, Vec<u8>>, Error = Error>> {
        let client = self.client.clone();
        let body = serialize_hashes(&refs);
        Box::new(
            future::result(self.request_for(Method::Post, GET_REFS_PATH))
                .map(|mut req| { req.set_body(body); req })
                .and_then(move |req| client.request(req))
                .map_err(|err| Error::Hyper(err))
                .and_then(retrieve_body)
                .map(|c| c.to_vec())
                .map(move |chunk| {
                    let mut values = HashMap::with_capacity(refs.len());
                    let mut i = 0;
                    while i != chunk.len() {
                        let hash = Hash::from_slice(&chunk[i..i + BYTE_LEN]);
                        i += BYTE_LEN;
                        let len = NetworkEndian::read_u32(&chunk[i..i + 4]) as usize;
                        i += 4;
                        let bytes = chunk[i..i + len].to_vec();
                        i += len;
                        values.insert(hash, bytes);
                    }
                    values
                })
        )
    }

    pub fn post_has_refs<'a>(&self, database: &'a ChunkStore, refs: HashSet<Hash>) -> Box<Future<Item = HashMap<Hash, bool>, Error = Error>> {
        let client = self.client.clone();
        let body = serialize_hashes(&refs);
        Box::new(
            future::result(self.request_for(Method::Post, HAS_REFS_PATH))
                .map(|mut req| { req.set_body(body); req }) // TODO: request_with_body?
                .and_then(move |req| client.request(req))
                .map_err(|err| Error::Hyper(err))
                .and_then(retrieve_body)
                .and_then(|c| String::from_utf8(c.to_vec()).map_err(|e| e.into()))
                .and_then(move |strs| {
                    let mut exists: HashMap<Hash, bool> = refs.iter().map(|r| (r.clone(), false)).collect();
                    for line in strs.lines() {
                        exists.insert(Hash::from_string(line)?, true);
                    }
                    Ok(exists)
                })
        )
    }
}

fn serialize_hashes(hashes: &HashSet<Hash>) -> Vec<u8> {
    let mut body = vec![0; 4];
    NetworkEndian::write_u32(&mut body[..], hashes.len() as u32);
    for hash in hashes {
        body.extend_from_slice(&hash.raw_bytes());
    }
    body
}

fn retrieve_body(res: Response) -> Box<Future<Item = hyper::Chunk, Error = Error>> {
    match res.status() {
        StatusCode::Ok => Box::new(res.body().concat2().map_err(|err| Error::Hyper(err))),
        status => Box::new(future::result(Err(Error::Http(status)))),
    }
}
