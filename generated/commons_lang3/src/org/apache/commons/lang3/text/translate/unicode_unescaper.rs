use java::io::IOException;
use java::io::Writer;

pub struct UnicodeUnescaper;

impl UnicodeUnescaper {
	pub fn new() -> org::apache::commons::lang3::text::translate::unicode_unescaper::UnicodeUnescaper {
	// empty
	}

	pub fn translate(&self, input: &/* Java */ java::lang::CharSequence /**/, index: i32, out: &/* Java */ java::io::Writer /**/) /* thrown(java.io.IOException | java.lang.IllegalArgumentException) */ -> i32 {
		if input.charAt(index) == '\\' && index + 1 < input.length() && input.charAt(index + 1) == 'u' {
			// consume optional additional 'u' chars
			let i: i32 = 2;
			while index + i < input.length() && input.charAt(index + i) == 'u' {
				i += 1;
			}
			if index + i < input.length() && input.charAt(index + i) == '+' {
				i += 1;
			}
			if index + i + 4 <= input.length() {
				// Get 4 hex digits
				/* final */ let unicode: CharSequence = input.subSequence(index + i, index + i + 4);
				let r0 = 'try0: {
					/* final */ let value: i32 = Integer::parseInt(&unicode.toString(), 16);
					out.write(value as char);
					break 'try0 Ok(());
				};
				match r0 {
					Err(e @ NumberFormatException) => {
						break 'try0 Err(IllegalArgumentException::new("Unable to parse unicode value: " + unicode, nfe));
					},
					Err(e) => Err(e)?,
					Ok => (),
				}
				return i + 4;
			}
			return Err(IllegalArgumentException::new("Less than 4 hex digits in unicode value: '" + input.subSequence(index, &input.length()) + "' due to end of CharSequence"));
		}
		return 0;
	}
}