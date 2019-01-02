#![warn(missing_docs)]

//! Copyright 2019
//! Licensed under the WTFPL License, Version 2.0 <http://www.wtfpl.net/>
use std::fs::File;
use std::io::BufReader;

use serde_derive;

/// Configuration struct 
#[derive(Debug, Deserialize)]
pub struct Config {
	/// The configuration name
	/// Used for error and information messages
	name: String,
	
	/// The path to backup
	path: String,

	/// The compress algorithm
	compress: Option<String>,

	/// Pre backup actions
	pre_backup: Option<Vec<Action>>,

	/// Post backup actions
	post_backup: Option<Vec<Action>>
}

/// Action struct
/// 
/// Actions can run before and after backups
#[derive(Debug, Deserialize)]
pub struct Action {
	/// The command to run in this action
	command: String,

	/// The backup should wait the action finish before continue with backups? 
	#[serde(rename = "action")]
	wait: String
}

/// Try to parse the specified file to a backuper configuration
/// 
/// # Arguments
/// 
/// * `path` - The path to for deserialization
pub fn parse_config(path : String) -> Result<Vec<Config>, String> {
	// Open the file in read-only mode with buffer
	let file = match File::open(path) {
		Ok(f) => f,
		Err(e) => return Err(format!("Error opening the file: {}", e))
	};

	let reader = BufReader::new(file);

	let cfg : Vec<Config> = match serde_json::from_reader(reader) {
		Ok(c) => c,
		Err(e) => return Err(format!("Error parsing the file: {}", e))
	};

	Ok(cfg)
}