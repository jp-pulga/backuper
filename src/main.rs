#![crate_name = "backuper"]
#![warn(missing_docs)]

//! Copyright 2019
//! Licensed under the WTFPL License, Version 2.0 <http://www.wtfpl.net/>

pub mod backup;
pub mod compressors;

use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};

fn main() {
	let matches = app_from_crate!()
		.arg(
			Arg::with_name("file")
				.help("Set the configuration file to load")
				.takes_value(true)
				.short("f")
				.long("file")
				.default_value("backuper.json"),
		)
		.arg(
			Arg::with_name("config")
				.help("The name of the configuration to backup")
				.takes_value(true)
				.short("c")
				.long("config")
				.default_value("*"),
		)
		.get_matches();

	let cfg = backup::backup::parse_config(matches.value_of("file").unwrap().to_string()).unwrap();
	let filter = matches.value_of("config").unwrap().to_string();

	for c in cfg {
		if c.is_valid_path() && (filter == "*" || c.name == filter) {
			c.do_backup();

			println!("The task '{}' is sucefull backuped!", c.name)
		} else {
			println!("Cannot find the path for the configuration '{}'", c.name)
		}
	}
}
