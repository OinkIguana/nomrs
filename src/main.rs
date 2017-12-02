//! This main function serves as a playground to test the library. Soon it will be replaced by
//! actual test suites
//!
//! The derive crate needs real some tests too.

extern crate nomrs;
#[macro_use] extern crate nomrs_derive;

use nomrs::{Noms, Database};
use nomrs::value::NomsList;

#[derive(Clone, Debug, FromNoms, IntoNoms, NomsStruct)]
struct Row {
    count_female: String,
    count_male: String,
}

#[derive(Clone, Debug, FromNoms, IntoNoms, NomsStruct)]
struct Meta {
    date: String,
    input_file: String,
}

fn main() {
    let noms = Noms::new();
    let db = noms.database()
        .noms_version("7.18")
        .http("localhost:8000")
        .unwrap();

    println!("{:?}", db.datasets().unwrap());
    let commit = db.dataset::<Meta, NomsList<Row>>("test").unwrap().head().unwrap();
    println!("{:?}", commit.meta());
    println!("{:?}", commit.parents().to_set());
    println!("{:?}", commit.value().to_vec());
    println!("{:?}", db.value_from("Hello world"));
}
