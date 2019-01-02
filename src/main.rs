#![crate_name = "backuper"]
#![warn(missing_docs)]

//! Copyright 2019
//! Licensed under the WTFPL License, Version 2.0 <http://www.wtfpl.net/>

#[macro_use]
extern crate serde_derive;

/// Configuration module
pub mod config;

fn main() {
	let cfg = config::parse_config("backuper.json".to_string()).unwrap();
	for c in cfg {
		println!("{:#?}", c);
	}
}
