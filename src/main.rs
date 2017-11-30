//! This main function serves as a playground to test the library. Soon it will be replaced by
//! actual test suites

extern crate nomrs;

use std::collections::HashMap;
use nomrs::{Noms, Database};
use nomrs::value::{NomsList, Empty};

fn main() {
    let noms = Noms::new();
    let db = noms.database()
        .noms_version("7.18")
        .http("localhost:8000")
        .unwrap();
    println!("{:?}", db.datasets().unwrap());
    println!("{:?}", db.dataset::<Empty, NomsList>("test").unwrap().head_value());
}
