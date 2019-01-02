use std::fs::File;
use std::io::BufReader;

use serde_derive;

#[derive(Deserialize)]
pub struct Config {
	name: String,
	path: String,
	compress: Option<String>,
	pre_backup: Option<Vec<String>>,
	post_backup: Option<Vec<String>>
}

pub fn parse_config(path : String) -> Result<Config, &'static str> {
	// Open the file in read-only mode with buffer.
	let file = match File::open(path) {
		Ok(f) => f,
		Err(_) => return Err("Cannot open the configuration file")
	};

	let reader = BufReader::new(file);

	let cfg : Config = match serde_json::from_reader(reader) {
		Ok(c) => c,
		Err(_) => return Err("Cannot read the file")
	};

	Ok(cfg)
}