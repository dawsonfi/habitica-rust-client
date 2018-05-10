#![cfg_attr(test, feature(proc_macro))]
#[cfg(test)]
extern crate mocktopus;

#[macro_use]
extern crate serde_json;
extern crate serde;
extern crate reqwest;

pub mod task;