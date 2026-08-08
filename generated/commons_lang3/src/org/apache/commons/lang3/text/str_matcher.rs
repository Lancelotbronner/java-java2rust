use java::util::Arrays;
use crate::org::apache::commons::lang3::ArraySorter;
use crate::org::apache::commons::lang3::ArrayUtils;
use crate::org::apache::commons::lang3::StringUtils;

pub struct StrMatcher;

impl StrMatcher {
	static COMMA_MATCHER: org::apache::commons::lang3::text::str_matcher::StrMatcher = CharMatcher::new(',');

	static TAB_MATCHER: org::apache::commons::lang3::text::str_matcher::StrMatcher = CharMatcher::new('\t');

	static SPACE_MATCHER: org::apache::commons::lang3::text::str_matcher::StrMatcher = CharMatcher::new(' ');

	static SPLIT_MATCHER: org::apache::commons::lang3::text::str_matcher::StrMatcher = CharSetMatcher::new(&" \t\n\r\f".toCharArray());

	static TRIM_MATCHER: org::apache::commons::lang3::text::str_matcher::StrMatcher = TrimMatcher::new();

	static SINGLE_QUOTE_MATCHER: org::apache::commons::lang3::text::str_matcher::StrMatcher = CharMatcher::new('\'');

	static DOUBLE_QUOTE_MATCHER: org::apache::commons::lang3::text::str_matcher::StrMatcher = CharMatcher::new('"');

	static QUOTE_MATCHER: org::apache::commons::lang3::text::str_matcher::StrMatcher = CharSetMatcher::new(&"'\"".toCharArray());

	static NONE_MATCHER: org::apache::commons::lang3::text::str_matcher::StrMatcher = NoMatcher::new();

	pub fn char_matcher(&self, ch: u16) -> org::apache::commons::lang3::text::str_matcher::StrMatcher {
		return CharMatcher::new(ch);
	}

	pub fn char_set_matcher(&self, chars: u16) -> org::apache::commons::lang3::text::str_matcher::StrMatcher {
		if ArrayUtils.isEmpty(chars) {
			return self.NONE_MATCHER;
		}
		if chars.length == 1 {
			return CharMatcher::new(chars[0]);
		}
		return CharSetMatcher::new(chars);
	}

	pub fn char_set_matcher(&self, chars: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_matcher::StrMatcher {
		if StringUtils::is_empty(chars) {
			return self.NONE_MATCHER;
		}
		if chars.length() == 1 {
			return CharMatcher::new(&chars.charAt(0));
		}
		return CharSetMatcher::new(&chars.toCharArray());
	}

	pub fn comma_matcher(&self) -> org::apache::commons::lang3::text::str_matcher::StrMatcher {
		return self.COMMA_MATCHER;
	}

	pub fn double_quote_matcher(&self) -> org::apache::commons::lang3::text::str_matcher::StrMatcher {
		return self.DOUBLE_QUOTE_MATCHER;
	}

	pub fn none_matcher(&self) -> org::apache::commons::lang3::text::str_matcher::StrMatcher {
		return self.NONE_MATCHER;
	}

	pub fn quote_matcher(&self) -> org::apache::commons::lang3::text::str_matcher::StrMatcher {
		return self.QUOTE_MATCHER;
	}

	pub fn single_quote_matcher(&self) -> org::apache::commons::lang3::text::str_matcher::StrMatcher {
		return self.SINGLE_QUOTE_MATCHER;
	}

	pub fn space_matcher(&self) -> org::apache::commons::lang3::text::str_matcher::StrMatcher {
		return self.SPACE_MATCHER;
	}

	pub fn split_matcher(&self) -> org::apache::commons::lang3::text::str_matcher::StrMatcher {
		return self.SPLIT_MATCHER;
	}

	pub fn string_matcher(&self, str: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_matcher::StrMatcher {
		if StringUtils::is_empty(str) {
			return self.NONE_MATCHER;
		}
		return StringMatcher::new(str);
	}

	pub fn tab_matcher(&self) -> org::apache::commons::lang3::text::str_matcher::StrMatcher {
		return self.TAB_MATCHER;
	}

	pub fn trim_matcher(&self) -> org::apache::commons::lang3::text::str_matcher::StrMatcher {
		return self.TRIM_MATCHER;
	}

	fn new() -> org::apache::commons::lang3::text::str_matcher::StrMatcher {
	}

	pub fn is_match(&self, buffer: &&[u16], pos: i32) -> i32 {
		return self.is_match(buffer, pos, 0, buffer.length);
	}

	pub fn is_match(&self, buffer: &&[u16], pos: i32, buffer_start: i32, buffer_end: i32) -> i32 ;
}

struct CharMatcher {
	ch: u16,
}

impl CharMatcher {
	fn new(ch: u16) -> org::apache::commons::lang3::text::str_matcher::CharMatcher {
		self.ch = ch;
	}

	pub fn is_match(&self, buffer: &&[u16], pos: i32, buffer_start: i32, buffer_end: i32) -> i32 {
		return  if self.ch == buffer[pos] { 1 } else { 0 };
	}
}

struct CharSetMatcher {
	chars: &[u16],
}

impl CharSetMatcher {
	fn new(chars: &&[u16]) -> org::apache::commons::lang3::text::str_matcher::CharSetMatcher {
		self.chars = ArraySorter.sort(&chars.clone());
	}

	pub fn is_match(&self, buffer: &&[u16], pos: i32, buffer_start: i32, buffer_end: i32) -> i32 {
		return  if Arrays::binarySearch(self.chars, buffer[pos]) >= 0 { 1 } else { 0 };
	}
}

struct NoMatcher;

impl NoMatcher {
	fn new() -> org::apache::commons::lang3::text::str_matcher::NoMatcher {
	}

	pub fn is_match(&self, buffer: &&[u16], pos: i32, buffer_start: i32, buffer_end: i32) -> i32 {
		return 0;
	}
}

struct StringMatcher {
	chars: &[u16],
}

impl StringMatcher {
	fn new(str: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::text::str_matcher::StringMatcher {
		self.chars = str.toCharArray();
	}

	pub fn is_match(&self, buffer: &&[u16], pos: i32, buffer_start: i32, buffer_end: i32) -> i32 {
		/* final */ let len: i32 = self.chars.length;
		if pos + len > buffer_end {
			return 0;
		}
		 {
			let i: i32 = 0;
			while i < self.chars.length {
				{
					if self.chars[i] != buffer[pos] {
						return 0;
					}
				}
				i += 1;
				pos += 1;
			 }
		 }
	
		return len;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return super.toString() + ' ' + Arrays::toString(self.chars);
	}
}

struct TrimMatcher;

impl TrimMatcher {
	fn new() -> org::apache::commons::lang3::text::str_matcher::TrimMatcher {
	}

	pub fn is_match(&self, buffer: &&[u16], pos: i32, buffer_start: i32, buffer_end: i32) -> i32 {
		return  if buffer[pos] <= 32 { 1 } else { 0 };
	}
}