use java::io::IOException;
use java::io::Writer;
use crate::org::apache::commons::lang3::CharUtils;

pub struct OctalUnescaper;

impl OctalUnescaper {
	pub fn new() -> org::apache::commons::lang3::text::translate::octal_unescaper::OctalUnescaper {
	// empty
	}

	fn is_zero_to_three(&self, ch: u16) -> bool {
		return ch >= '0' && ch <= '3';
	}

	pub fn translate(&self, input: &/* Java */ java::lang::CharSequence /**/, index: i32, out: &/* Java */ java::io::Writer /**/) /* thrown(java.io.IOException) */ -> i32 {
		// how many characters left, ignoring the first \
		/* final */ let remaining: i32 = input.length() - index - 1;
		/* final */ let builder: StringBuilder = StringBuilder::new();
		if input.charAt(index) == '\\' && remaining > 0 && CharUtils::is_octal(&input.charAt(index + 1)) {
			/* final */ let next: i32 = index + 1;
			/* final */ let next2: i32 = index + 2;
			/* final */ let next3: i32 = index + 3;
			// we know this is good as we checked it in the if block above
			builder.append(&input.charAt(next));
			if remaining > 1 && CharUtils::is_octal(&input.charAt(next2)) {
				builder.append(&input.charAt(next2));
				if remaining > 2 && self.is_zero_to_three(&input.charAt(next)) && CharUtils::is_octal(&input.charAt(next3)) {
					builder.append(&input.charAt(next3));
				}
			}
			out.write(&Integer::parseInt(&builder.toString(), 8));
			return 1 + builder.length();
		}
		return 0;
	}
}