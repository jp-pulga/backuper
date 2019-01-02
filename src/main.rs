#![crate_name = "backuper"]
#![warn(missing_docs)]

//! Copyright 2019
//! Licensed under the WTFPL License, Version 2.0 <http://www.wtfpl.net/>

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate clap;

pub mod config;

use clap::Arg;

fn main() {
	let matches = app_from_crate!()
		.arg(
			Arg::with_name("config")
				.help("Set the configuration file to load")
				.takes_value(true)
				.short("c")
				.long("config")
				.default_value("backuper.json"),
		)
		.get_matches();

	let cfg = config::parse_config(matches.value_of("config").unwrap().to_string()).unwrap();
	for c in cfg {
		if config::is_valid_path(&c) {
			config::run_pre_backup_tasks(&c);
			config::do_backup(&c);
			config::run_post_backup_tasks(&c);

			println!("The task '{}' is sucefull backuped!", c.name)
		} else {
			println!("Cannot find the path for the configuration '{}'", c.name)
		}
	}
}
