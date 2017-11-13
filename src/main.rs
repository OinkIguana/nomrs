//! This main function serves as a playground to test the library. Soon it will be replaced by
//! actual test suites

extern crate nomrs;

use nomrs::Noms;

fn main() {
    let noms = Noms::new();
    let db = noms.database()
        .http("localhost:8000")
        .noms_version("7.18")
        .build().unwrap();
    println!("{:?}", db.datasets().unwrap());
    println!("{:?}", db.dataset("test").unwrap().head_value());
}
