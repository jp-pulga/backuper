//! Compressor mod
//! All compressors going to be here

pub mod gzip;
pub mod uncompressed;

use serde_derive::Deserialize;
use std::boxed::Box;
use std::path::Path;

use crate::compressors::gzip::Gzip;
use crate::compressors::uncompressed::Uncompressed;

/// The type of compression used for backup files
#[derive(Copy, Clone, Debug, Deserialize)]
pub enum CompressType {
	/// Dont comrpess anithing
	None = 0,

	/// Zip with deflate compress
	Zip = 1,

	/// Zip with lmza2 compress
	SevenZip = 2,

	/// BZip with deflate compress
	Bzip = 3,

	/// GZip with deflate compress
	Gzip = 4,
}

type CompressResult = std::io::Result<()>;

/// Comprensable trait
pub trait Comprensable {
	/// Compress some data to backp destination
	fn compress(&self, org: &Path, dest: &Path) -> CompressResult;
}

/// Get the compress by its type
pub fn get_compress_by_type(t: Option<CompressType>) -> Box<Comprensable + 'static> {
	if t.is_none() {
		let c: Uncompressed = Default::default();
		return Box::new(c);
	}

	match t.unwrap() {
		CompressType::None => {
			let c: Uncompressed = Default::default();
			return Box::new(c);
		}
		CompressType::Gzip => {
			let c: Gzip = Default::default();
			return Box::new(c);
		}
		_ => {
			let c: Uncompressed = Default::default();
			return Box::new(c);
		}
	}
}
