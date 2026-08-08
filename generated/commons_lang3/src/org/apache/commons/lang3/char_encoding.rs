use java::nio::charset::Charset;
use java::nio::charset::IllegalCharsetNameException;
use java::nio::charset::StandardCharsets;

pub struct CharEncoding;

impl CharEncoding {
	pub static ISO_8859_1: /* Java */ java::lang::String /**/ = StandardCharsets::ISO_8859_1.name();

	pub static US_ASCII: /* Java */ java::lang::String /**/ = StandardCharsets::US_ASCII.name();

	pub static UTF_16: /* Java */ java::lang::String /**/ = StandardCharsets::UTF_16.name();

	pub static UTF_16BE: /* Java */ java::lang::String /**/ = StandardCharsets::UTF_16BE.name();

	pub static UTF_16LE: /* Java */ java::lang::String /**/ = StandardCharsets::UTF_16LE.name();

	pub static UTF_8: /* Java */ java::lang::String /**/ = StandardCharsets::UTF_8.name();

	pub fn is_supported(&self, name: &/* Java */ java::lang::String /**/) -> bool {
		if name == null {
			return false;
		}
		let r0 = 'try0: {
			return Charset::isSupported(name);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ IllegalCharsetNameException) => {
				return false;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn new() -> org::apache::commons::lang3::char_encoding::CharEncoding {
	// empty
	}
}