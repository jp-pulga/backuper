//! Mod for help file system operations

use std::fs;
use std::io::Error as IOError;
use std::io::ErrorKind;
use std::io::Result as IOResult;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};

/// Copy the origina path to de destination path
/// If the origin path is an folder, it will pe recursive copied
///
/// # Arguments
///
/// * `origin` - The origin path
/// * `destination` - The destination path
///
/// # Remarks
/// This function assumes that the origin path is valid and exists
pub fn copy(origin: &String, destination: &String) -> IOResult<()> {
	let org_path = Path::new(origin);

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

	let mut v: Vec<PathBuf> = vec![origin.to_path_buf()];
	let mut size: usize = v.len();
	let mut i: usize = 0;
	let base_path = origin.to_str().unwrap();

	while {
		if let Some(r) = process_dir(base_path, &v[i], &destination)? {
			size += r.len();
			v.extend(r);
		}

		i = i + 1;
		i != size
	} {}

	Ok(())
}

fn process_dir(
	base_path: &str,
	origin: &Path,
	destination: &String,
) -> IOResult<Option<Vec<PathBuf>>> {
	ensure_sane_dir_path(destination).expect("Cannot ensure sane destination path");
	create_dir_if_not_exists(destination)
		.expect(&format!("Cannot create the path {}", destination));

	let mut result: Vec<PathBuf> = Vec::new();
	for entry in fs::read_dir(&origin)? {
		let e = entry?;
		let path = e.path();

		if path.is_dir() {
			result.push(e.path());
			continue;
		}

		let dest_sub_folder_path = format!(
			"{}{}{}",
			destination,
			MAIN_SEPARATOR,
			origin.strip_prefix(base_path).unwrap().display()
		);
		create_dir_if_not_exists(&dest_sub_folder_path)
			.expect(&format!("Cannot create the path {}", dest_sub_folder_path));

		fs::copy(
			path,
			format!(
				"{}{}{}",
				dest_sub_folder_path,
				MAIN_SEPARATOR,
				e.path()
					.strip_prefix(origin)
					.expect("Cannot get subfolder path")
					.display()
			),
		)?;
	}

	if result.len() == 0 {
		return Ok(None);
	}

	Ok(Some(result))
}

fn ensure_sane_dir_path(path_to_check: &String) -> IOResult<()> {
	let path = Path::new(path_to_check);

	if path.exists() && path.is_file() {
		return Err(IOError::new(
			ErrorKind::Other,
			format!(
				"The destination folder [{}] already exists as an file!",
				path_to_check
			),
		));
	}

	Ok(())
}

fn create_dir_if_not_exists(path_to_check: &String) -> IOResult<()> {
	let path = Path::new(path_to_check);
	if !path.exists() {
		fs::create_dir(&path_to_check)?;
	}

	Ok(())
}
