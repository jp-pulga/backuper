//! This the gzip compresion
//! We only need move the folder specified to peform a backup

use crate::compressors::{Comprensable, CompressResult};
use std::path::Path;

/// Struct for handle uncomprensed data
#[derive(Copy, Clone, Default)]
pub struct Gzip;

impl Comprensable for Gzip {
	fn compress(&self, org: &Path, dest: &Path) -> CompressResult {
		println!("Coping {} to {}", org.display(), dest.display());

		Ok(())
	}
}
