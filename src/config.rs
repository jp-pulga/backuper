//! Copyright 2019
//! Licensed under the WTFPL License, Version 2.0 <http://www.wtfpl.net/>
use std::fs::File;
use std::io::BufReader;

use serde_derive;

/// Configuration struct
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
	/// The configuration name
	/// Used for error and information messages
	pub name: String,

	/// The path to backup
	path: String,

	/// The compress algorithm
	compress: Option<String>,

	/// Pre backup actions
	pre_backup: Option<Vec<Action>>,

	/// Post backup actions
	post_backup: Option<Vec<Action>>,
}

impl Config {
	/// Check if the path specified in the configuration file is valid
	pub fn is_valid_path(&self) -> bool {
		std::path::Path::new(&self.path).exists()
	}
}

/// Action struct
///
/// Actions can run before and after backups
#[derive(Clone, Debug, Deserialize)]
pub struct Action {
	/// The command to run in this action
	command: String,

	/// The backup should wait the action finish before continue with backups?
	wait: bool,
}

/// Try to parse the specified file to a backuper configuration
///
/// # Arguments
///
/// * `path` - The path to for deserialization
pub fn parse_config(path: String) -> Result<Vec<Config>, String> {
	match File::open(path) {
		Ok(f) => match serde_json::from_reader(BufReader::new(f)) {
			Ok(v) => return Ok(v),
			Err(e) => return Err(format!("Error parsing the file: {}", e)),
		},
		Err(e) => return Err(format!("Error opening the file: {}", e)),
	}
}
