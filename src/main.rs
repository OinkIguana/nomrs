//! This main function serves as a playground to test the library. Soon it will be replaced by
//! actual test suites

extern crate nomrs;
#[macro_use] extern crate nomrs_derive;

use std::collections::HashMap;
use nomrs::{Noms, Database};
use nomrs::value::{NomsList, Empty};

#[derive(Clone, Debug, FromNoms, IntoNoms, NomsStruct)]
struct Row {
    count_female: String,
    count_male: String,
}

fn main() {
    let noms = Noms::new();
    let db = noms.database()
        .noms_version("7.18")
        .http("localhost:8000")
        .unwrap();
    println!("{:?}", db.datasets().unwrap());
    println!("{:?}", db.dataset::<Empty, NomsList<Row>>("test").unwrap().head_value().unwrap().to_vec());
}
