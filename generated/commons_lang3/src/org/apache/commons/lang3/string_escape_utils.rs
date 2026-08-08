use java::io::IOException;
use java::io::Writer;
use crate::org::apache::commons::lang3::text::translate::AggregateTranslator;
use crate::org::apache::commons::lang3::text::translate::CharSequenceTranslator;
use crate::org::apache::commons::lang3::text::translate::EntityArrays;
use crate::org::apache::commons::lang3::text::translate::JavaUnicodeEscaper;
use crate::org::apache::commons::lang3::text::translate::LookupTranslator;
use crate::org::apache::commons::lang3::text::translate::NumericEntityEscaper;
use crate::org::apache::commons::lang3::text::translate::NumericEntityUnescaper;
use crate::org::apache::commons::lang3::text::translate::OctalUnescaper;
use crate::org::apache::commons::lang3::text::translate::UnicodeUnescaper;
use crate::org::apache::commons::lang3::text::translate::UnicodeUnpairedSurrogateRemover;

pub struct StringEscapeUtils;

impl StringEscapeUtils {
	pub static ESCAPE_JAVA: org::apache::commons::lang3::text::translate::char_sequence_translator::CharSequenceTranslator = LookupTranslator::new(: [[Option<String>; ]; ] = [[None; ]; ]).with(LookupTranslator::new(&EntityArrays::jav_a_ctr_l_char_s_escape())).with(&JavaUnicodeEscaper::outside_of(32, 0x7f));

	pub static ESCAPE_ECMASCRIPT: org::apache::commons::lang3::text::translate::char_sequence_translator::CharSequenceTranslator = AggregateTranslator::new(LookupTranslator::new(: [[Option<String>; ]; ] = [[None; ]; ]), LookupTranslator::new(&EntityArrays::jav_a_ctr_l_char_s_escape()), &JavaUnicodeEscaper::outside_of(32, 0x7f));

	pub static ESCAPE_JSON: org::apache::commons::lang3::text::translate::char_sequence_translator::CharSequenceTranslator = AggregateTranslator::new(LookupTranslator::new(: [[Option<String>; ]; ] = [[None; ]; ]), LookupTranslator::new(&EntityArrays::jav_a_ctr_l_char_s_escape()), &JavaUnicodeEscaper::outside_of(32, 0x7f));

	pub static ESCAPE_XML: org::apache::commons::lang3::text::translate::char_sequence_translator::CharSequenceTranslator = AggregateTranslator::new(LookupTranslator::new(&EntityArrays::basi_c_escape()), LookupTranslator::new(&EntityArrays::apo_s_escape()));

	pub static ESCAPE_XML10: org::apache::commons::lang3::text::translate::char_sequence_translator::CharSequenceTranslator = AggregateTranslator::new(LookupTranslator::new(&EntityArrays::basi_c_escape()), LookupTranslator::new(&EntityArrays::apo_s_escape()), LookupTranslator::new(: [[Option<String>; ]; ] = [[None; ]; ]), &NumericEntityEscaper::between(0x7f, 0x84), &NumericEntityEscaper::between(0x86, 0x9f), UnicodeUnpairedSurrogateRemover::new());

	pub static ESCAPE_XML11: org::apache::commons::lang3::text::translate::char_sequence_translator::CharSequenceTranslator = AggregateTranslator::new(LookupTranslator::new(&EntityArrays::basi_c_escape()), LookupTranslator::new(&EntityArrays::apo_s_escape()), LookupTranslator::new(: [[Option<String>; ]; ] = [[None; ]; ]), &NumericEntityEscaper::between(0x1, 0x8), &NumericEntityEscaper::between(0xe, 0x1f), &NumericEntityEscaper::between(0x7f, 0x84), &NumericEntityEscaper::between(0x86, 0x9f), UnicodeUnpairedSurrogateRemover::new());

	pub static ESCAPE_HTML3: org::apache::commons::lang3::text::translate::char_sequence_translator::CharSequenceTranslator = AggregateTranslator::new(LookupTranslator::new(&EntityArrays::basi_c_escape()), LookupTranslator::new(&EntityArrays::is_o8859_1_escape()));

	pub static ESCAPE_HTML4: org::apache::commons::lang3::text::translate::char_sequence_translator::CharSequenceTranslator = AggregateTranslator::new(LookupTranslator::new(&EntityArrays::basi_c_escape()), LookupTranslator::new(&EntityArrays::is_o8859_1_escape()), LookupTranslator::new(&EntityArrays::htm_l40_extende_d_escape()));

	pub static ESCAPE_CSV: org::apache::commons::lang3::text::translate::char_sequence_translator::CharSequenceTranslator = CsvEscaper::new();

	pub static UNESCAPE_JAVA: org::apache::commons::lang3::text::translate::char_sequence_translator::CharSequenceTranslator = AggregateTranslator::new(// .between('\1', '\377'),
	OctalUnescaper::new(), UnicodeUnescaper::new(), LookupTranslator::new(&EntityArrays::jav_a_ctr_l_char_s_unescape()), LookupTranslator::new(: [[Option<String>; ]; ] = [[None; ]; ]));

	pub static UNESCAPE_ECMASCRIPT: org::apache::commons::lang3::text::translate::char_sequence_translator::CharSequenceTranslator = UNESCAPE_JAVA;

	pub static UNESCAPE_JSON: org::apache::commons::lang3::text::translate::char_sequence_translator::CharSequenceTranslator = UNESCAPE_JAVA;

	pub static UNESCAPE_HTML3: org::apache::commons::lang3::text::translate::char_sequence_translator::CharSequenceTranslator = AggregateTranslator::new(LookupTranslator::new(&EntityArrays::basi_c_unescape()), LookupTranslator::new(&EntityArrays::is_o8859_1_unescape()), NumericEntityUnescaper::new());

	pub static UNESCAPE_HTML4: org::apache::commons::lang3::text::translate::char_sequence_translator::CharSequenceTranslator = AggregateTranslator::new(LookupTranslator::new(&EntityArrays::basi_c_unescape()), LookupTranslator::new(&EntityArrays::is_o8859_1_unescape()), LookupTranslator::new(&EntityArrays::htm_l40_extende_d_unescape()), NumericEntityUnescaper::new());

	pub static UNESCAPE_XML: org::apache::commons::lang3::text::translate::char_sequence_translator::CharSequenceTranslator = AggregateTranslator::new(LookupTranslator::new(&EntityArrays::basi_c_unescape()), LookupTranslator::new(&EntityArrays::apo_s_unescape()), NumericEntityUnescaper::new());

	pub static UNESCAPE_CSV: org::apache::commons::lang3::text::translate::char_sequence_translator::CharSequenceTranslator = CsvUnescaper::new();

	pub fn escape_csv(&self, input: &/* Java */ java::lang::String /**/) /* thrown(java.io.UncheckedIOException) */ -> /* Java */ java::lang::String /**/ {
		return self.ESCAPE_CSV.translate(input)?;
	}

	pub fn escape_ecma_script(&self, input: &/* Java */ java::lang::String /**/) /* thrown(java.io.UncheckedIOException) */ -> /* Java */ java::lang::String /**/ {
		return self.ESCAPE_ECMASCRIPT.translate(input)?;
	}

	pub fn escape_html3(&self, input: &/* Java */ java::lang::String /**/) /* thrown(java.io.UncheckedIOException) */ -> /* Java */ java::lang::String /**/ {
		return self.ESCAPE_HTML3.translate(input)?;
	}

	pub fn escape_html4(&self, input: &/* Java */ java::lang::String /**/) /* thrown(java.io.UncheckedIOException) */ -> /* Java */ java::lang::String /**/ {
		return self.ESCAPE_HTML4.translate(input)?;
	}

	pub fn escape_java(&self, input: &/* Java */ java::lang::String /**/) /* thrown(java.io.UncheckedIOException) */ -> /* Java */ java::lang::String /**/ {
		return self.ESCAPE_JAVA.translate(input)?;
	}

	pub fn escape_json(&self, input: &/* Java */ java::lang::String /**/) /* thrown(java.io.UncheckedIOException) */ -> /* Java */ java::lang::String /**/ {
		return self.ESCAPE_JSON.translate(input)?;
	}

	pub fn escape_xml(&self, input: &/* Java */ java::lang::String /**/) /* thrown(java.io.UncheckedIOException) */ -> /* Java */ java::lang::String /**/ {
		return self.ESCAPE_XML.translate(input)?;
	}

	pub fn escape_xml10(&self, input: &/* Java */ java::lang::String /**/) /* thrown(java.io.UncheckedIOException) */ -> /* Java */ java::lang::String /**/ {
		return self.ESCAPE_XML10.translate(input)?;
	}

	pub fn escape_xml11(&self, input: &/* Java */ java::lang::String /**/) /* thrown(java.io.UncheckedIOException) */ -> /* Java */ java::lang::String /**/ {
		return self.ESCAPE_XML11.translate(input)?;
	}

	pub fn unescape_csv(&self, input: &/* Java */ java::lang::String /**/) /* thrown(java.io.UncheckedIOException) */ -> /* Java */ java::lang::String /**/ {
		return self.UNESCAPE_CSV.translate(input)?;
	}

	pub fn unescape_ecma_script(&self, input: &/* Java */ java::lang::String /**/) /* thrown(java.io.UncheckedIOException) */ -> /* Java */ java::lang::String /**/ {
		return self.UNESCAPE_ECMASCRIPT.translate(input)?;
	}

	pub fn unescape_html3(&self, input: &/* Java */ java::lang::String /**/) /* thrown(java.io.UncheckedIOException) */ -> /* Java */ java::lang::String /**/ {
		return self.UNESCAPE_HTML3.translate(input)?;
	}

	pub fn unescape_html4(&self, input: &/* Java */ java::lang::String /**/) /* thrown(java.io.UncheckedIOException) */ -> /* Java */ java::lang::String /**/ {
		return self.UNESCAPE_HTML4.translate(input)?;
	}

	pub fn unescape_java(&self, input: &/* Java */ java::lang::String /**/) /* thrown(java.io.UncheckedIOException) */ -> /* Java */ java::lang::String /**/ {
		return self.UNESCAPE_JAVA.translate(input)?;
	}

	pub fn unescape_json(&self, input: &/* Java */ java::lang::String /**/) /* thrown(java.io.UncheckedIOException) */ -> /* Java */ java::lang::String /**/ {
		return self.UNESCAPE_JSON.translate(input)?;
	}

	pub fn unescape_xml(&self, input: &/* Java */ java::lang::String /**/) /* thrown(java.io.UncheckedIOException) */ -> /* Java */ java::lang::String /**/ {
		return self.UNESCAPE_XML.translate(input)?;
	}

	pub fn new() -> org::apache::commons::lang3::string_escape_utils::StringEscapeUtils {
	// empty
	}
}

struct CsvEscaper;

impl CsvEscaper {
	static CSV_DELIMITER: u16 = ',';

	static CSV_QUOTE: u16 = '"';

	static CSV_QUOTE_STR: /* Java */ java::lang::String /**/ = String::valueOf(CSV_QUOTE);

	static CSV_SEARCH_CHARS: &[u16] = vec![CSV_DELIMITER, CSV_QUOTE, CharUtils::CR, CharUtils::LF, ]
	;

	pub fn translate(&self, input: &/* Java */ java::lang::CharSequence /**/, index: i32, out: &/* Java */ java::io::Writer /**/) /* thrown(java.io.IOException | java.lang.IllegalStateException) */ -> i32 {
		if index != 0 {
			return Err(IllegalStateException::new("CsvEscaper should never reach the [1] index"));
		}
		if StringUtils::contains_none(&input.toString(), self.CSV_SEARCH_CHARS) {
			out.write(&input.toString());
		} else {
			out.write(self.CSV_QUOTE);
			out.write(&Strings::org::apache::commons::lang3::strings::Strings::CS.replace(&input.toString(), self.CSV_QUOTE_STR, self.CSV_QUOTE_STR + self.CSV_QUOTE_STR));
			out.write(self.CSV_QUOTE);
		}
		return Character::codePointCount(input, 0, &input.length());
	}
}

struct CsvUnescaper;

impl CsvUnescaper {
	static CSV_DELIMITER: u16 = ',';

	static CSV_QUOTE: u16 = '"';

	static CSV_QUOTE_STR: /* Java */ java::lang::String /**/ = String::valueOf(CSV_QUOTE);

	static CSV_SEARCH_CHARS: &[u16] = vec![CSV_DELIMITER, CSV_QUOTE, CharUtils::CR, CharUtils::LF, ]
	;

	pub fn translate(&self, input: &/* Java */ java::lang::CharSequence /**/, index: i32, out: &/* Java */ java::io::Writer /**/) /* thrown(java.io.IOException | java.lang.IllegalStateException) */ -> i32 {
		if index != 0 {
			return Err(IllegalStateException::new("CsvUnescaper should never reach the [1] index"));
		}
		if input.charAt(0) != self.CSV_QUOTE || input.charAt(input.length() - 1) != self.CSV_QUOTE {
			out.write(&input.toString());
			return Character::codePointCount(input, 0, &input.length());
		}
		// strip quotes
		/* final */ let quoteless: String = input.subSequence(1, input.length() - 1).toString();
		if StringUtils::contains_any(quoteless, self.CSV_SEARCH_CHARS) {
			// deal with escaped quotes; ie) ""
			out.write(&Strings::org::apache::commons::lang3::strings::Strings::CS.replace(quoteless, self.CSV_QUOTE_STR + self.CSV_QUOTE_STR, self.CSV_QUOTE_STR));
		} else {
			out.write(&input.toString());
		}
		return Character::codePointCount(input, 0, &input.length());
	}
}