use java::nio::charset::Charset;
use java::nio::charset::UnsupportedCharsetException;

struct Charsets;

impl Charsets {
	fn to_charset(&self, charset: &/* Java */ java::nio::charset::Charset /**/) -> /* Java */ java::nio::charset::Charset /**/ {
		return  if charset == null { Charset::defaultCharset() } else { charset };
	}

	fn to_charset(&self, charset_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::nio::charset::Charset /**/ {
		return  if charset_name == null { Charset::defaultCharset() } else { Charset::forName(charset_name) };
	}

	fn to_charset_name(&self, charset_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return  if charset_name == null { Charset::defaultCharset().name() } else { charset_name };
	}
}