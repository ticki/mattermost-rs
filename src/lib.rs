#![feature(slice_concat_ext)]
#![feature(plugin)]
#![plugin(json_macros)]

mod outgoing;
pub mod server;
pub mod payload;

extern crate hyper;
extern crate rustc_serialize;
