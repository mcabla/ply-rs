//#![cfg(feature = "serde_impl")]
//extern crate linked_hash_map;

mod test_grammar {
    include!(concat!(env!("OUT_DIR"), "/ply_grammar.rs"));
}

use self::test_grammar as grammar;
mod parser;
mod ply;