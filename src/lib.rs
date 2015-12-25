#![feature(slice_concat_ext)]
#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

mod outgoing;
pub mod server;
pub mod payload;

extern crate hyper;
extern crate serde;
extern crate serde_json;
