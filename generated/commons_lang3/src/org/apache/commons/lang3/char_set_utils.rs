use crate::org::apache::commons::lang3::stream::Streams;

pub struct CharSetUtils;

impl CharSetUtils {
	pub fn contains_any(&self, str: &/* Java */ java::lang::String /**/, set: &/* Java */ java::lang::String /**/) -> bool {
		if StringUtils::is_empty(str) || org::apache::commons::lang3::char_set_utils::CharSetUtils::deep_empty(set) {
			return false;
		}
		/* final */ let chars: CharSet = CharSet::get_instance(set);
		for /* final */ c in str.toCharArray() {
			if chars.contains(c) {
				return true;
			}
		}
		return false;
	}

	pub fn count(&self, str: &/* Java */ java::lang::String /**/, set: &/* Java */ java::lang::String /**/) -> i32 {
		if StringUtils::is_empty(str) || org::apache::commons::lang3::char_set_utils::CharSetUtils::deep_empty(set) {
			return 0;
		}
		/* final */ let chars: CharSet = CharSet::get_instance(set);
		let count: i32 = 0;
		for /* final */ c in str.toCharArray() {
			if chars.contains(c) {
				count += 1;
			}
		}
		return count;
	}

	fn deep_empty(&self, strings: &&[/* Java */ java::lang::String /**/]) -> bool {
		return Streams::of(strings).allMatch(StringUtils::isEmpty);
	}

	pub fn delete(&self, str: &/* Java */ java::lang::String /**/, set: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if StringUtils::is_empty(str) || org::apache::commons::lang3::char_set_utils::CharSetUtils::deep_empty(set) {
			return str;
		}
		return org::apache::commons::lang3::char_set_utils::CharSetUtils::modify(str, set, false);
	}

	pub fn keep(&self, str: &/* Java */ java::lang::String /**/, set: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if str == null {
			return null;
		}
		if str.isEmpty() || org::apache::commons::lang3::char_set_utils::CharSetUtils::deep_empty(set) {
			return StringUtils::EMPTY;
		}
		return org::apache::commons::lang3::char_set_utils::CharSetUtils::modify(str, set, true);
	}

	fn modify(&self, str: &/* Java */ java::lang::String /**/, set: &&[/* Java */ java::lang::String /**/], expect: bool) -> /* Java */ java::lang::String /**/ {
		/* final */ let chars: CharSet = CharSet::get_instance(set);
		/* final */ let buffer: StringBuilder = StringBuilder::new(&str.length());
		/* final */ let chrs: Vec<char> = str.toCharArray();
		for /* final */ chr in chrs {
			if chars.contains(chr) == expect {
				buffer.append(chr);
			}
		}
		return buffer.toString();
	}

	pub fn squeeze(&self, str: &/* Java */ java::lang::String /**/, set: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if StringUtils::is_empty(str) || org::apache::commons::lang3::char_set_utils::CharSetUtils::deep_empty(set) {
			return str;
		}
		/* final */ let chars: CharSet = CharSet::get_instance(set);
		/* final */ let buffer: StringBuilder = StringBuilder::new(&str.length());
		/* final */ let chrs: Vec<char> = str.toCharArray();
		/* final */ let sz: i32 = chrs.length;
		let last_char: char = chrs[0];
		let ch: char;
		let in_chars: Character = null;
		let not_in_chars: Character = null;
		buffer.append(last_char);
		 {
			let i: i32 = 1;
			while i < sz {
				{
					ch = chrs[i];
					if ch == last_char {
						if in_chars != null && ch == in_chars {
							continue;
						}
						if not_in_chars == null || ch != not_in_chars {
							if chars.contains(ch) {
								in_chars = ch;
								continue;
							}
							not_in_chars = ch;
						}
					}
					buffer.append(ch);
					last_char = ch;
				}
				i += 1;
			 }
		 }
	
		return buffer.toString();
	}

	pub fn new() -> org::apache::commons::lang3::char_set_utils::CharSetUtils {
	}
}