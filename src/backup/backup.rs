//! Backup module

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::Command;

use serde_derive::Deserialize;
use walkdir::WalkDir;
use zip;

use crate::compressors::*;

/// This structs store all we need to know
/// about some backup
#[derive(Debug, Deserialize)]
pub struct Backup {
	/// The name of the current backup configuration
	/// Used for error and information messages
	pub name: String,

	/// The path to backup
	pub path: PathBuf,

	/// The destination for all backups files
	pub destination: PathBuf,

	/// The compress algorithm
	compress: Option<CompressType>,

	/// Pre backup actions
	pre_backup: Option<Vec<Action>>,

	/// Post backup actions
	post_backup: Option<Vec<Action>>,
}

/// Action struct
///
/// Actions can run before and after backups
#[derive(Debug, Deserialize)]
pub struct Action {
	/// The command to run in this action
	command: String,

	/// The backup should wait the action finish before continue with backups?
	wait: bool,
}

impl Backup {
	/// Check if the path specified in the configuration file is valid
	pub fn is_valid_path(&self) -> bool {
		self.path.as_path().exists()
	}

	/// Run all pre-build task registred for the backup
	pub fn run_pre_backup_tasks(&self) {
		match &self.pre_backup {
			Some(values) => self.run_actions(values),
			None => (),
		}
	}

	/// Run all post-build task registred for the backup
	pub fn run_post_backup_tasks(&self) {
		match &self.post_backup {
			Some(values) => self.run_actions(values),
			None => (),
		}
	}

	/// Do the backup and crompress
	pub fn do_backup(&self) {
		self.run_pre_backup_tasks();

		let base_org_path = &self.path.as_path();
		let mut c = get_compress_by_type(self.compress);
		c.init(&self);

		for dir in WalkDir::new(&self.path).follow_links(true) {
			let entry = dir.unwrap();
			let mut base_dest_path = self.destination.clone();
			base_dest_path.push(entry.path().strip_prefix(base_org_path).unwrap());

			c.compress(&entry.path(), &base_dest_path.as_path())
				.unwrap();
		}

		c.finish();
		self.run_post_backup_tasks();
	}

	fn run_actions(&self, actions: &Vec<Action>) {
		for v in actions {
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
}

/// Try to parse the specified file to a backuper configuration
///
/// # Arguments
///
/// * `path` - The path to for deserialization
pub fn parse_config(path: String) -> Result<Vec<Backup>, String> {
	match File::open(path) {
		Ok(f) => match serde_json::from_reader(BufReader::new(f)) {
			Ok(v) => return Ok(v),
			Err(e) => return Err(format!("Error parsing the file: {}", e)),
		},
		Err(e) => return Err(format!("Error opening the file: {}", e)),
	}
}
