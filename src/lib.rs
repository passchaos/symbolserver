//! This crate implements symbol handling for system libraries
#![recursion_limit = "1024"]

#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate serde_xml;
#[macro_use] extern crate error_chain;
extern crate zip;
extern crate walkdir;
extern crate uuid;
extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate mach_object;
extern crate memmap;
extern crate clap;
extern crate pbr;
extern crate xz2;
extern crate tempfile;
extern crate humansize;
extern crate rusoto;
extern crate chrono;
extern crate hyper;
extern crate hyper_native_tls;
extern crate url;
extern crate md5;
extern crate log;
extern crate rustc_serialize;
#[macro_use] extern crate if_chain;

pub use errors::{Result, Error, ErrorKind, ResultExt};

mod macros;
mod errors;
mod memdbdump;
mod memdbtypes;
mod memdbstash;
mod utils;
mod config;
mod s3;

pub mod cli;
pub mod dsym;
pub mod sdk;
pub mod memdb;
