//! Copyright 2019
//! Licensed under the WTFPL License, Version 2.0 <http://www.wtfpl.net/>
use std::fs::File;
use std::io::BufReader;
use std::process::Command;

use crate::utils;
use serde_derive;

/// Configuration struct
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
	/// The configuration name
	/// Used for error and information messages
	pub name: String,

	/// The path to backup
	path: String,

	/// The destination for all backups files
	destination: String,

	/// The compress algorithm
	compress: Option<String>,

	/// Pre backup actions
	pre_backup: Option<Vec<Action>>,

	/// Post backup actions
	post_backup: Option<Vec<Action>>,
}

/// Check if the path specified in the configuration file is valid
pub fn is_valid_path(cfg: &Config) -> bool {
	std::path::Path::new(&cfg.path).exists()
}

/// Run all pre-build task registred for the backup
pub fn run_pre_backup_tasks(cfg: &Config) {
	match &cfg.pre_backup {
		Some(values) => {
			for v in values {
				if v.wait {
					Command::new(&v.command)
						.spawn()
						.expect("Error spwaining the process")
						.wait()
						.expect("Error in process");
				} else {
					Command::new(&v.command)
						.spawn()
						.expect("Error spwaining the process");
				}
			}
		}
		None => (),
	}
}

/// Run all post-build task registred for the backup
pub fn run_post_backup_tasks(cfg: &Config) {
	match &cfg.post_backup {
		Some(values) => {
			for v in values {
				if v.wait {
					Command::new(&v.command)
						.spawn()
						.expect("Error spwaining the process")
						.wait()
						.expect("Error in process");
				} else {
					Command::new(&v.command)
						.spawn()
						.expect("Error spwaining the process");
				}
			}
		}
		None => (),
	}
}

/// Do the backup and crompress
pub fn do_backup(cfg: &Config) {
	match &cfg.compress {
		Some(v) => match v.len() {
			0 => do_uncompressed_bakcup(cfg),
			_ => (),
		},
		None => do_uncompressed_bakcup(cfg),
	};
}

/// Do the uncompressed backup
/// AKA: Just copy the folder to destination
fn do_uncompressed_bakcup(cfg: &Config) {
	utils::fs::copy(&cfg.path, &cfg.destination).expect("Error in backup");
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
