//! Mod for help file system operations

use std::fs;
use std::io::Error as IOError;
use std::io::ErrorKind;
use std::io::Result as IOResult;
use std::path::{Path, MAIN_SEPARATOR};

/// Copy the origina path to de destination path
/// If the origin path is an folder, it will pe recursive copied
///
/// # Arguments
///
/// * `origin` - The origin path
/// * `destination` - The destination path
pub fn copy(origin: &String, destination: &String) -> IOResult<()> {
	let org_path = Path::new(origin);
	if !org_path.exists() {
		return Err(IOError::new(
			ErrorKind::NotFound,
			"Cannot found the specified path",
		));
	}

	if org_path.is_file() {
		fs::copy(origin, destination)?;
	} else {
		copy_folder(org_path, destination)?;
	}

	return Ok(());
}

/// Copy an entire folder
///
/// # Arguments
///
/// * `origin` - The origin path
/// * `destination` - The destination path
fn copy_folder(origin: &Path, destination: &String) -> IOResult<()> {
	ensure_sane_dir_path(destination).expect("Cannot ensure sane destination path");

	for entry in fs::read_dir(origin)? {
		let e = entry?;
		let path = e.path();

		if path.is_dir() {
			let dest_sub_folder_path = format!(
				"{}{}{}",
				destination,
				MAIN_SEPARATOR,
				path.strip_prefix(origin)
					.expect("Cannot get subfolder path")
					.display()
			);

			copy_folder(&path, &dest_sub_folder_path)?;
		} else {
			fs::copy(
				path,
				format!(
					"{}{}{}",
					destination,
					MAIN_SEPARATOR,
					e.file_name().into_string().unwrap()
				),
			)?;
		}
	}

	Ok(())
}

fn ensure_sane_dir_path(path_to_check: &String) -> IOResult<()> {
	let path = Path::new(path_to_check);

	if path.exists() && path.is_file() {
		return Err(IOError::new(
			ErrorKind::Other,
			"THe destination folder already exists as an file!",
		));
	}

	if !path.exists() {
		fs::create_dir(&path_to_check)?;
	}

	return Ok(());
}
