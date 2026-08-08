use java::io::IOException;
use java::io::Writer;

pub struct UnicodeEscaper {
	below: i32,
	above: i32,
	between: bool,
}

impl UnicodeEscaper {
	pub fn above(&self, code_point: i32) -> org::apache::commons::lang3::text::translate::unicode_escaper::UnicodeEscaper {
		return org::apache::commons::lang3::text::translate::unicode_escaper::UnicodeEscaper::outside_of(0, code_point);
	}

	pub fn below(&self, code_point: i32) -> org::apache::commons::lang3::text::translate::unicode_escaper::UnicodeEscaper {
		return org::apache::commons::lang3::text::translate::unicode_escaper::UnicodeEscaper::outside_of(code_point, Integer::MAX_VALUE);
	}

	pub fn between(&self, code_point_low: i32, code_point_high: i32) -> org::apache::commons::lang3::text::translate::unicode_escaper::UnicodeEscaper {
		return UnicodeEscaper::new(code_point_low, code_point_high, true);
	}

	pub fn outside_of(&self, code_point_low: i32, code_point_high: i32) -> org::apache::commons::lang3::text::translate::unicode_escaper::UnicodeEscaper {
		return UnicodeEscaper::new(code_point_low, code_point_high, false);
	}

	pub fn new() -> org::apache::commons::lang3::text::translate::unicode_escaper::UnicodeEscaper {
		this(0, Integer::MAX_VALUE, true);
	}

	fn new(below: i32, above: i32, between: bool) -> org::apache::commons::lang3::text::translate::unicode_escaper::UnicodeEscaper {
		self.below = below;
		self.above = above;
		self.between = between;
	}

	fn to_utf16_escape(&self, code_point: i32) -> /* Java */ java::lang::String /**/ {
		return "\\u" + org::apache::commons::lang3::text::translate::char_sequence_translator::CharSequenceTranslator::hex(code_point);
	}

	pub fn translate(&self, code_point: i32, out: &/* Java */ java::io::Writer /**/) /* thrown(java.io.IOException) */ -> bool {
		if self.between {
			if code_point < self.below || code_point > self.above {
				return false;
			}
		} else if code_point >= self.below && code_point <= self.above {
			return false;
		}
		// TODO: Handle potential + sign per various Unicode escape implementations
		if code_point > 0xffff {
			out.write(&self.to_utf16_escape(code_point));
		} else {
			out.write("\\u");
			out.write([code_point /* signed */ >> 12 & 15]);
			out.write([code_point /* signed */ >> 8 & 15]);
			out.write([code_point /* signed */ >> 4 & 15]);
			out.write([code_point & 15]);
		}
		return true;
	}
}