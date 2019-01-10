//! Compressor mod
//! All compressors going to be here

pub mod compressed;
pub mod uncompressed;

use serde_derive::Deserialize;
use std::boxed::Box;
use std::path::Path;

use crate::backup::backup::Backup;
use crate::compressors::compressed::Zip;
use crate::compressors::uncompressed::Uncompressed;

/// The type of compression used for backup files
#[derive(Copy, Clone, Debug, Deserialize)]
pub enum CompressType {
	/// Zip with deflate compress
	Zip = 1,

	/// BZip with deflate compress
	Bzip = 2,
}

type CompressResult = std::io::Result<()>;

/// Comprensable trait
pub trait Comprensable {
	/// Init the
	fn init(&mut self, bkp: &Backup);

	/// Compress some data to backp destination
	fn compress(&mut self, org: &Path, dest: &Path) -> CompressResult;

	/// Finish the compression
	fn finish(&mut self);
}

/// Get the compress by its type
pub fn get_compress_by_type(t: Option<CompressType>) -> Box<Comprensable + 'static> {
	if t.is_none() {
		let c: Uncompressed = Default::default();
		return Box::new(c);
	}

	let mut c: Zip = Default::default();
	match t.unwrap() {
		CompressType::Zip => {
			c.options = Some(
				zip::write::FileOptions::default()
					.compression_method(zip::CompressionMethod::Deflated)
					.unix_permissions(0o755),
			);
		}
		CompressType::Bzip => {
			c.options = Some(
				zip::write::FileOptions::default()
					.compression_method(zip::CompressionMethod::Bzip2)
					.unix_permissions(0o755),
			);
		}
	}
	return Box::new(c);
}
