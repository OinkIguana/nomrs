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
extern crate either;

pub mod database;
pub mod dataset;
pub mod value;
pub mod error;
pub mod util;

// TODO: make a prelude of some sort...
pub use database::Database;
pub use chunk::Chunk;

mod http;
mod chunk;
mod hash;

use std::cell::RefCell;
use std::rc::Rc;
use tokio_core::reactor::Core;
use database::DatabaseBuilder;

struct InnerNoms {
    event_loop: Core,
}

#[derive(Clone)]
pub struct Noms(Rc<RefCell<InnerNoms>>);

impl Noms {
    pub fn new() -> Noms {
        Noms(Rc::new(RefCell::new(InnerNoms{ event_loop: Core::new().unwrap() })))
    }

    pub fn database(&self) -> DatabaseBuilder {
        DatabaseBuilder::new(self.0.clone())
    }
}
