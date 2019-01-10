//! This the uncomprensed "compresion"
//! We only need move the folder specified to peform a backup

use crate::backup::backup::Backup;
use crate::compressors::{Comprensable, CompressResult};

use std::fs::{copy, create_dir};
use std::path::Path;

/// Struct for handle uncomprensed data
#[derive(Copy, Clone, Default)]
pub struct Uncompressed;

impl Comprensable for Uncompressed {
	fn init(&mut self, _bkp: &Backup) {}

	fn compress(&mut self, org: &Path, dest: &Path) -> CompressResult {
		println!("Coping {} to {}", org.display(), dest.display());

		if org.is_dir() {
			create_dir(dest)?;
		} else {
			copy(org, dest)?;
		}

		Ok(())
	}

	fn finish(&mut self) {}
}
