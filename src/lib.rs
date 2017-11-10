//! This crate provides a wrapper around the Noms HTTP server API.
//!
//! TODO: more description
//! TODO: documentation
//! TODO: examples

#[macro_use] extern crate lazy_static;
extern crate byteorder;
extern crate crypto;
#[macro_use] extern crate hyper;
extern crate tokio_core;
extern crate futures;
extern crate data_encoding;

pub mod database;
pub mod dataset;
pub mod value;
pub mod error;

mod http;
mod codec;
mod chunk;
mod hash;

use tokio_core::reactor::Core;
use database::DatabaseBuilder;

pub struct Noms {
    event_loop: Core,
}

impl Noms {
    pub fn new() -> Self {
        Self{ event_loop: Core::new().unwrap() }
    }

    pub fn database(&mut self) -> DatabaseBuilder {
        DatabaseBuilder::new(self)
    }
}
