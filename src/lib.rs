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

pub mod database;
pub mod dataset;
pub mod value;
pub mod error;

mod http;
mod codec;
mod chunk;
mod hash;
