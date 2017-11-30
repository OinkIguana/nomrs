//! This main function serves as a playground to test the library. Soon it will be replaced by
//! actual test suites

#![feature(associated_consts)]

extern crate nomrs;

use std::collections::HashMap;
use nomrs::{Noms, Database, Chunk};
use nomrs::value::{FromNoms, IntoNoms, NomsStruct, NomsValue, NomsList, Empty};

#[derive(Clone, Copy, Debug)]
struct Row {
    count_female: u64,
    count_male: u64,
}
// TODO: make a derivation for these
impl<'a> FromNoms<'a> for Row {
    fn from_noms(chunk: &Chunk) -> Self {
        NomsValue::from_noms(chunk).transform_struct()
    }
}
impl IntoNoms for Row {
    fn into_noms(&self) -> Vec<u8> {
        unimplemented!();
    }
}
impl<'a> NomsStruct<'a> for Row {
    const NAME: &'static str = "Row";
    fn from_prop_list(mut props: HashMap<String, NomsValue<'a>>) -> Option<Self> {
        println!("{:?}", props);
        Some(Row{
            count_male: props.remove("countMale")?.transform(),
            count_female: props.remove("countFemale")?.transform(),
        })
    }
    fn to_prop_list(&self) -> HashMap<String, NomsValue<'a>> {
        HashMap::new()
    }
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
