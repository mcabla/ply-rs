extern crate ply_rs_bw;

use std::time::Instant;
use ply_rs_bw as ply;

/// Demonstrates simplest use case for reading from a file.
fn main() {
    // set up a reader, in this a file.
    let i = Instant::now();
    let path = "/home/audrius/opw/deleted/link_1.ply";
    let mut f = std::fs::File::open(path).unwrap();


    // create a parser
    let p = ply::parser::Parser::<ply::ply::DefaultElement>::new();

    // use the parser: read the entire file
    let ply = p.read_ply(&mut f);

    // make sure it did work
    assert!(ply.is_ok());
    println!("Elapsed {:?}", i.elapsed());

    // proof that data has been read
    //println!("Read ply data: {:#?}", ply.unwrap());
}
