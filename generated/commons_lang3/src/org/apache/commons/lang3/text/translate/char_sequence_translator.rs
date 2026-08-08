use java::io::IOException;
use java::io::StringWriter;
use java::io::UncheckedIOException;
use java::io::Writer;
use java::util::Locale;
use java::util::Objects;
use crate::org::apache::commons::lang3::ArrayUtils;

pub struct CharSequenceTranslator;

impl CharSequenceTranslator {
	static HEX_DIGITS: &[u16] = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', ]
	;

	pub fn hex(&self, code_point: i32) -> /* Java */ java::lang::String /**/ {
		return Integer::toHexString(code_point).toUpperCase(Locale::ENGLISH);
	}

	pub fn new() -> org::apache::commons::lang3::text::translate::char_sequence_translator::CharSequenceTranslator {
	// empty
	}

	pub fn translate(&self, input: &/* Java */ java::lang::CharSequence /**/) /* thrown(java.io.UncheckedIOException) */ -> /* Java */ java::lang::String /**/ {
		if input == null {
			return null;
		}
		let r0 = 'try0: {
			/* final */ let writer: StringWriter = StringWriter::new(input.length() * 2);
			self.translate(input, writer);
			return writer.toString();
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ IOException) => {
				// this should never ever happen while writing to a StringWriter
				break 'try0 Err(UncheckedIOException::new(ioe));
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn translate(&self, input: &/* Java */ java::lang::CharSequence /**/, index: i32, out: &/* Java */ java::io::Writer /**/) /* thrown(java.io.IOException) */ -> i32 ;

	pub fn translate(&self, input: &/* Java */ java::lang::CharSequence /**/, writer: &/* Java */ java::io::Writer /**/) /* thrown(java.io.IOException) */ {
		Objects::requireNonNull(writer, "writer");
		if input == null {
			return;
		}
		let pos: i32 = 0;
		/* final */ let len: i32 = input.length();
		while pos < len {
			/* final */ let consumed: i32 = self.translate(input, pos, writer)?;
			if consumed == 0 {
				// inlined implementation of Character.toChars(Character.codePointAt(input, pos))
				// avoids allocating temp char arrays and duplicate checks
				/* final */ let c1: char = input.charAt(pos);
				writer.write(c1);
				pos += 1;
				if Character::isHighSurrogate(c1) && pos < len {
					/* final */ let c2: char = input.charAt(pos);
					if Character::isLowSurrogate(c2) {
						writer.write(c2);
						pos += 1;
					}
				}
				continue;
			}
			// and they just took care of a surrogate pair
			 {
				let pt: i32 = 0;
				while pt < consumed {
					{
						pos += Character::charCount(&Character::codePointAt(input, pos));
					}
					pt += 1;
				 }
			 }
	
		}
	}

	pub fn with(&self, translators: &org::apache::commons::lang3::text::translate::char_sequence_translator::CharSequenceTranslator) -> org::apache::commons::lang3::text::translate::char_sequence_translator::CharSequenceTranslator {
		/* final */ let new_array: [Option<CharSequenceTranslator>; translators.length + 1] = [None; translators.length + 1];
		new_array[0] = self;
		return AggregateTranslator::new(&ArrayUtils::arraycopy(translators, 0, new_array, 1, translators.length));
	}
}