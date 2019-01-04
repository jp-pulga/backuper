//! Mod for help file system operations

use std::fs;
use std::io::Error as IOError;
use std::io::ErrorKind;
use std::io::Result as IOResult;
use std::path::{Path, PathBuf};

/// Copy the origin path to de destination path
/// If the origin path is an folder, it will pe recursive copied
///
/// # Arguments
///
/// * `origin` - The origin path
/// * `destination` - The destination path
///
/// # Remarks
/// This function assumes that the origin path is valid and exists
pub fn copy(origin: &Path, destination: &Path) -> IOResult<()> {
	if origin.is_file() {
		fs::copy(origin, destination)?;
	} else {
		copy_folder(origin, destination)?;
	}

	return Ok(());
}

/// Copy an entire folder
///
/// # Arguments
///
/// * `origin` - The origin path
/// * `destination` - The destination path
fn copy_folder(origin: &Path, destination: &Path) -> IOResult<()> {
	ensure_sane_dir_path(destination)?;

	let mut vec: Vec<PathBuf> = vec![origin.to_path_buf()];

	while {
		let to_backup = vec.pop().unwrap();
		let mut base_dest_path = PathBuf::from(destination);
		base_dest_path.push(&to_backup.as_path().strip_prefix(origin).unwrap());

		if let Some(r) = process_dir(&to_backup, &mut base_dest_path)? {
			vec.extend(r);
		}

		vec.len() > 0
	} {}

	Ok(())
}

/// Process files and folders in some directory
///
/// # Arguments
///
/// * `origin` - The origin path
/// * `destination` - The destination path
///
/// # Returns
/// All the folders inside this folders to process later
fn process_dir(origin: &Path, destination: &mut PathBuf) -> IOResult<Option<Vec<PathBuf>>> {
	ensure_sane_dir_path(destination)?;

	let mut result: Vec<PathBuf> = Vec::new();
	for entry in fs::read_dir(&origin)? {
		let e = entry?;
		let path = e.path();

		if path.is_dir() {
			result.push(e.path());
			continue;
		}

		create_dir_if_not_exists(&destination.as_path())?;

		destination.push(&path.strip_prefix(origin).unwrap());
		fs::copy(&path, &destination)?;
	}

	if result.len() == 0 {
		return Ok(None);
	}

	Ok(Some(result))
}

/// Ensure we dont try to write in a file
fn ensure_sane_dir_path(path_to_check: &Path) -> IOResult<()> {
	if path_to_check.exists() && path_to_check.is_file() {
		return Err(IOError::new(
			ErrorKind::Other,
			format!(
				"The destination folder [{}] already exists as an file!",
				path_to_check.display()
			),
		));
	}

	Ok(())
}

/// Create the specified directory if the it not exist
fn create_dir_if_not_exists(path_to_check: &Path) -> IOResult<()> {
	if !path_to_check.exists() {
		fs::create_dir(&path_to_check)?;
	}

	Ok(())
}
