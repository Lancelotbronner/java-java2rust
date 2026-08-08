use java::util::regex::Matcher;
use java::util::regex::Pattern;
use crate::org::apache::commons::lang3::ArrayUtils;
use crate::org::apache::commons::lang3::StringUtils;

pub struct WordUtils;

impl WordUtils {
	pub fn capitalize(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return .capitalize(str, null);
	}

	pub fn capitalize(&self, str: &/* Java */ java::lang::String /**/, delimiters: u16) -> /* Java */ java::lang::String /**/ {
		/* final */ let delim_len: i32 =  if delimiters == null { -1 } else { delimiters.length };
		if StringUtils::is_empty(str) || delim_len == 0 {
			return str;
		}
		/* final */ let buffer: Vec<char> = str.toCharArray();
		let capitalize_next: bool = true;
		 {
			let i: i32 = 0;
			while i < buffer.length {
				{
					/* final */ let ch: char = buffer[i];
					if org::apache::commons::lang3::text::word_utils::WordUtils::is_delimiter(ch, delimiters) {
						capitalize_next = true;
					} else if capitalize_next {
						buffer[i] = Character::toTitleCase(ch);
						capitalize_next = false;
					}
				}
				i += 1;
			 }
		 }
	
		return String::new(buffer);
	}

	pub fn capitalize_fully(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return .capitalizeFully(str, null);
	}

	pub fn capitalize_fully(&self, str: &/* Java */ java::lang::String /**/, delimiters: u16) -> /* Java */ java::lang::String /**/ {
		/* final */ let delim_len: i32 =  if delimiters == null { -1 } else { delimiters.length };
		if StringUtils::is_empty(str) || delim_len == 0 {
			return str;
		}
		return org::apache::commons::lang3::text::word_utils::WordUtils::capitalize(&str.toLowerCase(), delimiters);
	}

	pub fn contains_all_words(&self, word: &/* Java */ java::lang::CharSequence /**/, words: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if StringUtils::is_empty(word) || ArrayUtils::is_empty(words) {
			return false;
		}
		for /* final */ w in words {
			if StringUtils::is_blank(w) {
				return false;
			}
			/* final */ let p: Pattern = Pattern::compile(".*\\b" + Pattern::quote(&w.toString()) + "\\b.*");
			if !p.matcher(word).matches() {
				return false;
			}
		}
		return true;
	}

	pub fn initials(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return .initials(str, null);
	}

	pub fn initials(&self, str: &/* Java */ java::lang::String /**/, delimiters: u16) -> /* Java */ java::lang::String /**/ {
		if StringUtils::is_empty(str) {
			return str;
		}
		if delimiters != null && delimiters.length == 0 {
			return StringUtils::EMPTY;
		}
		/* final */ let str_len: i32 = str.length();
		/* final */ let buf: [Option<char>; str_len / 2 + 1] = [None; str_len / 2 + 1];
		let count: i32 = 0;
		let last_was_gap: bool = true;
		 {
			let i: i32 = 0;
			while i < str_len {
				{
					/* final */ let ch: char = str.charAt(i);
					if org::apache::commons::lang3::text::word_utils::WordUtils::is_delimiter(ch, delimiters) {
						last_was_gap = true;
						// ignore ch
						continue;
					}
					if last_was_gap {
						buf[count += 1 !!!check!!! post increment] = ch;
						last_was_gap = false;
					}
				}
				i += 1;
			 }
		 }
	
		return String::new(buf, 0, count);
	}

	fn is_delimiter(&self, ch: u16, delimiters: &&[u16]) -> bool {
		return  if delimiters == null { Character::isWhitespace(ch) } else { ArrayUtils::contains(delimiters, ch) };
	}

	pub fn swap_case(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if StringUtils::is_empty(str) {
			return str;
		}
		/* final */ let buffer: Vec<char> = str.toCharArray();
		let whitespace: bool = true;
		 {
			let i: i32 = 0;
			while i < buffer.length {
				{
					/* final */ let ch: char = buffer[i];
					if Character::isUpperCase(ch) || Character::isTitleCase(ch) {
						buffer[i] = Character::toLowerCase(ch);
						whitespace = false;
					} else if Character::isLowerCase(ch) {
						if whitespace {
							buffer[i] = Character::toTitleCase(ch);
							whitespace = false;
						} else {
							buffer[i] = Character::toUpperCase(ch);
						}
					} else {
						whitespace = Character::isWhitespace(ch);
					}
				}
				i += 1;
			 }
		 }
	
		return String::new(buffer);
	}

	pub fn uncapitalize(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return .uncapitalize(str, null);
	}

	pub fn uncapitalize(&self, str: &/* Java */ java::lang::String /**/, delimiters: u16) -> /* Java */ java::lang::String /**/ {
		/* final */ let delim_len: i32 =  if delimiters == null { -1 } else { delimiters.length };
		if StringUtils::is_empty(str) || delim_len == 0 {
			return str;
		}
		/* final */ let buffer: Vec<char> = str.toCharArray();
		let uncapitalize_next: bool = true;
		 {
			let i: i32 = 0;
			while i < buffer.length {
				{
					/* final */ let ch: char = buffer[i];
					if org::apache::commons::lang3::text::word_utils::WordUtils::is_delimiter(ch, delimiters) {
						uncapitalize_next = true;
					} else if uncapitalize_next {
						buffer[i] = Character::toLowerCase(ch);
						uncapitalize_next = false;
					}
				}
				i += 1;
			 }
		 }
	
		return String::new(buffer);
	}

	pub fn wrap(&self, str: &/* Java */ java::lang::String /**/, wrap_length: i32) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::text::word_utils::WordUtils::wrap(str, wrap_length, null, false);
	}

	pub fn wrap(&self, str: &/* Java */ java::lang::String /**/, wrap_length: i32, new_line_str: &/* Java */ java::lang::String /**/, wrap_long_words: bool) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::text::word_utils::WordUtils::wrap(str, wrap_length, new_line_str, wrap_long_words, " ");
	}

	pub fn wrap(&self, str: &/* Java */ java::lang::String /**/, mut wrap_length: i32, mut new_line_str: &/* Java */ java::lang::String /**/, wrap_long_words: bool, mut wrap_on: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if str == null {
			return null;
		}
		if new_line_str == null {
			new_line_str = System::lineSeparator();
		}
		if wrap_length < 1 {
			wrap_length = 1;
		}
		if StringUtils::is_blank(wrap_on) {
			wrap_on = " ";
		}
		/* final */ let pattern_to_wrap_on: Pattern = Pattern::compile(wrap_on);
		/* final */ let input_line_length: i32 = str.length();
		let offset: i32 = 0;
		/* final */ let wrapped_line: StringBuilder = StringBuilder::new(input_line_length + 32);
		while offset < input_line_length {
			let space_to_wrap_at: i32 = -1;
			let matcher: Matcher = pattern_to_wrap_on.matcher(&str.substring(offset, &Math::min(Math::min(Integer::MAX_VALUE, offset + wrap_length + 1) as i32, input_line_length)));
			if matcher.find() {
				if matcher.start() == 0 {
					offset += matcher.end();
					continue;
				}
				space_to_wrap_at = matcher.start() + offset;
			}
			// only last line without leading spaces is left
			if input_line_length - offset <= wrap_length {
				break;
			}
			while matcher.find() {
				space_to_wrap_at = matcher.start() + offset;
			}
			if space_to_wrap_at >= offset {
				// normal case
				wrapped_line.append(str, offset, space_to_wrap_at);
				wrapped_line.append(new_line_str);
				offset = space_to_wrap_at + 1;
			} else // really long word or URL
			if wrap_long_words {
				// wrap really long word one line at a time
				wrapped_line.append(str, offset, wrap_length + offset);
				wrapped_line.append(new_line_str);
				offset += wrap_length;
			} else {
				// do not wrap really long word, just extend beyond limit
				matcher = pattern_to_wrap_on.matcher(&str.substring(offset + wrap_length));
				if matcher.find() {
					space_to_wrap_at = matcher.start() + offset + wrap_length;
				}
				if space_to_wrap_at >= 0 {
					wrapped_line.append(str, offset, space_to_wrap_at);
					wrapped_line.append(new_line_str);
					offset = space_to_wrap_at + 1;
				} else {
					wrapped_line.append(str, offset, &str.length());
					offset = input_line_length;
				}
			}
		}
		// Whatever is left in line is short enough to just pass through
		wrapped_line.append(str, offset, &str.length());
		return wrapped_line.toString();
	}

	pub fn new() -> org::apache::commons::lang3::text::word_utils::WordUtils {
	}
}