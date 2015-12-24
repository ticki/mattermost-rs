#![feature(slice_concat_ext)]
#![feature(plugin)]
#![plugin(json_macros)]

mod outgoing;
mod server;
mod payload;

extern crate hyper;
