//! This crate provides a wrapper around the Noms HTTP server API.
//!
//! TODO: more description
//! TODO: documentation
//! TODO: examples

extern crate byteorder;
extern crate crypto;

pub mod database;
pub mod dataset;
pub mod value;
pub mod error;

mod http;
mod codec;
mod chunk;
mod hash;
