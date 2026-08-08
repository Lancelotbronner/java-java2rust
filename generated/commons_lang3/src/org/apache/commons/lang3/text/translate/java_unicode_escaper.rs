pub struct JavaUnicodeEscaper;

impl JavaUnicodeEscaper {
	pub fn above(&self, code_point: i32) -> org::apache::commons::lang3::text::translate::java_unicode_escaper::JavaUnicodeEscaper {
		return org::apache::commons::lang3::text::translate::java_unicode_escaper::JavaUnicodeEscaper::outside_of(0, code_point);
	}

	pub fn below(&self, code_point: i32) -> org::apache::commons::lang3::text::translate::java_unicode_escaper::JavaUnicodeEscaper {
		return org::apache::commons::lang3::text::translate::java_unicode_escaper::JavaUnicodeEscaper::outside_of(code_point, Integer::MAX_VALUE);
	}

	pub fn between(&self, code_point_low: i32, code_point_high: i32) -> org::apache::commons::lang3::text::translate::java_unicode_escaper::JavaUnicodeEscaper {
		return JavaUnicodeEscaper::new(code_point_low, code_point_high, true);
	}

	pub fn outside_of(&self, code_point_low: i32, code_point_high: i32) -> org::apache::commons::lang3::text::translate::java_unicode_escaper::JavaUnicodeEscaper {
		return JavaUnicodeEscaper::new(code_point_low, code_point_high, false);
	}

	pub fn new(below: i32, above: i32, between: bool) -> org::apache::commons::lang3::text::translate::java_unicode_escaper::JavaUnicodeEscaper {
		super(below, above, between);
	}

	fn to_utf16_escape(&self, code_point: i32) -> /* Java */ java::lang::String /**/ {
		/* final */ let surrogate_pair: Vec<char> = Character::toChars(code_point);
		return "\\u" + org::apache::commons::lang3::text::translate::char_sequence_translator::CharSequenceTranslator::hex(surrogate_pair[0]) + "\\u" + org::apache::commons::lang3::text::translate::char_sequence_translator::CharSequenceTranslator::hex(surrogate_pair[1]);
	}
}