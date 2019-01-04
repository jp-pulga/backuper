//! Compressor mod
//! All compressors going to be here

/// The type of compression used for backup files
#[derive(Debug, Deserialize)]
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
