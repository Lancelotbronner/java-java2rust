use java::io::IOException;
use java::io::Writer;
use java::util::Arrays;
use java::util::Collections;
use java::util::EnumSet;
use crate::org::apache::commons::lang3::CharUtils;

pub struct NumericEntityUnescaper {
	options: /* Java */ java::util::EnumSet /**/,
}

impl NumericEntityUnescaper {
	pub fn new(options: &org::apache::commons::lang3::text::translate::numeric_entity_unescaper::OPTION) -> org::apache::commons::lang3::text::translate::numeric_entity_unescaper::NumericEntityUnescaper {
		if options.length > 0 {
			self.options = EnumSet::copyOf(&Arrays::asList(options));
		} else {
			self.options = EnumSet::copyOf(&Collections::singletonList(OPTION::semiColonRequired));
		}
	}

	pub fn is_set(&self, option: &org::apache::commons::lang3::text::translate::numeric_entity_unescaper::OPTION) -> bool {
		return self.options != null && self.options.contains(option);
	}

	pub fn translate(&self, input: &/* Java */ java::lang::CharSequence /**/, index: i32, out: &/* Java */ java::io::Writer /**/) /* thrown(java.io.IOException | java.lang.IllegalArgumentException) */ -> i32 {
		/* final */ let seq_end: i32 = input.length();
		// Uses -2 to ensure there is something after the &#
		if input.charAt(index) == '&' && index < seq_end - 2 && input.charAt(index + 1) == '#' {
			let start: i32 = index + 2;
			let is_hex: bool = false;
			/* final */ let first_char: char = input.charAt(start);
			if first_char == 'x' || first_char == 'X' {
				start += 1;
				is_hex = true;
				// Check there's more than just an x after the &#
				if start == seq_end {
					return 0;
				}
			}
			let end: i32 = start;
			// Note that this supports character codes without a ; on the end
			while end < seq_end && CharUtils::is_hex(&input.charAt(end)) {
				end += 1;
			}
			/* final */ let semi_next: bool = end != seq_end && input.charAt(end) == ';';
			if !semi_next {
				if self.is_set(OPTION::semiColonRequired) {
					return 0;
				}
				if self.is_set(OPTION::errorIfNoSemiColon) {
					return Err(IllegalArgumentException::new("Semi-colon required at end of numeric entity"));
				}
			}
			/* final */ let entity_value: i32;
			let r0 = 'try0: {
				if is_hex {
					entity_value = Integer::parseInt(&input.subSequence(start, end).toString(), 16);
				} else {
					entity_value = Integer::parseInt(&input.subSequence(start, end).toString(), 10);
				}
				break 'try0 Ok(());
			};
			match r0 {
				Err(e @ NumberFormatException) => {
					return 0;
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
			if entity_value > 0xFFFF {
				/* final */ let chars: Vec<char> = Character::toChars(entity_value);
				out.write(chars[0]);
				out.write(chars[1]);
			} else {
				out.write(entity_value);
			}
			return 2 + end - start + ( if is_hex { 1 } else { 0 }) + ( if semi_next { 1 } else { 0 });
		}
		return 0;
	}
}

pub enum OPTION;