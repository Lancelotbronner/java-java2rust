use crate::org::apache::commons::lang3::StringUtils::INDEX_NOT_FOUND;
use crate::org::apache::commons::lang3::builder::AbstractSupplier;
use crate::org::apache::commons::lang3::function::ToBooleanBiFunction;

pub struct Strings {
	ignore_case: bool,
	null_is_less: bool,
}

impl Strings {
	pub static CI: org::apache::commons::lang3::strings::Strings = CiStrings::new(true);

	pub static CS: org::apache::commons::lang3::strings::Strings = CsStrings::new(true);

	pub fn builder(&self) -> org::apache::commons::lang3::strings::Builder {
		return Builder::new();
	}

	fn contains_any(&self, test: &org::apache::commons::lang3::function::to_boolean_bi_function::ToBooleanBiFunction, cs: &/* Java */ java::lang::CharSequence /**/, search_char_sequences: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if StringUtils::is_empty(cs) || ArrayUtils::is_empty(search_char_sequences) {
			return false;
		}
		for /* final */ search_char_sequence in search_char_sequences {
			if test.apply_as_boolean(cs, search_char_sequence) {
				return true;
			}
		}
		return false;
	}

	fn eq(&self, o1: &/* Java */ java::lang::Object /**/, o2: &/* Java */ java::lang::Object /**/) -> bool {
		return  if o1 == null { o2 == null } else { o1.equals(o2) };
	}

	fn new(ignore_case: bool, null_is_less: bool) -> org::apache::commons::lang3::strings::Strings {
		self.ignoreCase = ignore_case;
		self.nullIsLess = null_is_less;
	}

	pub fn append_if_missing(&self, str: &/* Java */ java::lang::String /**/, suffix: &/* Java */ java::lang::CharSequence /**/, suffixes: &/* Java */ java::lang::CharSequence /**/) -> /* Java */ java::lang::String /**/ {
		if str == null || StringUtils::is_empty(suffix) || self.ends_with(str, suffix) {
			return str;
		}
		if ArrayUtils::is_not_empty(suffixes) {
			for /* final */ s in suffixes {
				if self.ends_with(str, s) {
					return str;
				}
			}
		}
		return str + suffix;
	}

	pub fn compare(&self, str1: &/* Java */ java::lang::String /**/, str2: &/* Java */ java::lang::String /**/) -> i32 ;

	pub fn contains(&self, seq: &/* Java */ java::lang::CharSequence /**/, search_seq: &/* Java */ java::lang::CharSequence /**/) -> bool ;

	pub fn contains_any(&self, cs: &/* Java */ java::lang::CharSequence /**/, search_char_sequences: &/* Java */ java::lang::CharSequence /**/) -> bool {
		return org::apache::commons::lang3::strings::Strings::contains_any(self::contains, cs, search_char_sequences);
	}

	pub fn ends_with(&self, str: &/* Java */ java::lang::CharSequence /**/, suffix: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if str == null || suffix == null {
			return str == suffix;
		}
		/* final */ let suf_len: i32 = suffix.length();
		if suf_len > str.length() {
			return false;
		}
		return CharSequenceUtils::region_matches(str, self.ignore_case, str.length() - suf_len, suffix, 0, suf_len);
	}

	pub fn ends_with_any(&self, sequence: &/* Java */ java::lang::CharSequence /**/, search_strings: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if StringUtils::is_empty(sequence) || ArrayUtils::is_empty(search_strings) {
			return false;
		}
		for /* final */ search_string in search_strings {
			if self.ends_with(sequence, search_string) {
				return true;
			}
		}
		return false;
	}

	pub fn equals(&self, cs1: &/* Java */ java::lang::CharSequence /**/, cs2: &/* Java */ java::lang::CharSequence /**/) -> bool ;

	pub fn equals(&self, str1: &/* Java */ java::lang::String /**/, str2: &/* Java */ java::lang::String /**/) -> bool ;

	pub fn equals_any(&self, string: &/* Java */ java::lang::CharSequence /**/, search_strings: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if ArrayUtils::is_not_empty(search_strings) {
			for /* final */ next in search_strings {
				if self.equals(string, next) {
					return true;
				}
			}
		}
		return false;
	}

	pub fn index_of(&self, seq: &/* Java */ java::lang::CharSequence /**/, search_seq: &/* Java */ java::lang::CharSequence /**/) -> i32 {
		return self.index_of(seq, search_seq, 0);
	}

	pub fn index_of(&self, seq: &/* Java */ java::lang::CharSequence /**/, search_seq: &/* Java */ java::lang::CharSequence /**/, start_pos: i32) -> i32 ;

	pub fn is_case_sensitive(&self) -> bool {
		return !self.ignore_case;
	}

	fn is_null_is_less(&self) -> bool {
		return self.null_is_less;
	}

	pub fn last_index_of(&self, str: &/* Java */ java::lang::CharSequence /**/, search_str: &/* Java */ java::lang::CharSequence /**/) -> i32 {
		if str == null {
			return ;
		}
		return self.last_index_of(str, search_str, &str.length());
	}

	pub fn last_index_of(&self, seq: &/* Java */ java::lang::CharSequence /**/, search_seq: &/* Java */ java::lang::CharSequence /**/, start_pos: i32) -> i32 ;

	pub fn prepend_if_missing(&self, str: &/* Java */ java::lang::String /**/, prefix: &/* Java */ java::lang::CharSequence /**/, prefixes: &/* Java */ java::lang::CharSequence /**/) -> /* Java */ java::lang::String /**/ {
		if str == null || StringUtils::is_empty(prefix) || self.starts_with(str, prefix) {
			return str;
		}
		if ArrayUtils::is_not_empty(prefixes) {
			for /* final */ p in prefixes {
				if self.starts_with(str, p) {
					return str;
				}
			}
		}
		return prefix + str;
	}

	pub fn remove(&self, str: &/* Java */ java::lang::String /**/, remove: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return self.replace(str, remove, StringUtils::EMPTY, -1);
	}

	pub fn remove_end(&self, str: &/* Java */ java::lang::String /**/, remove: &/* Java */ java::lang::CharSequence /**/) -> /* Java */ java::lang::String /**/ {
		if StringUtils::is_empty(str) || StringUtils::is_empty(remove) {
			return str;
		}
		if self.ends_with(str, remove) {
			return str.substring(0, str.length() - remove.length());
		}
		return str;
	}

	pub fn remove_start(&self, str: &/* Java */ java::lang::String /**/, remove: &/* Java */ java::lang::CharSequence /**/) -> /* Java */ java::lang::String /**/ {
		if str != null && self.starts_with(str, remove) {
			return str.substring(&StringUtils::length(remove));
		}
		return str;
	}

	pub fn replace(&self, text: &/* Java */ java::lang::String /**/, search_string: &/* Java */ java::lang::String /**/, replacement: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return self.replace(text, search_string, replacement, -1);
	}

	pub fn replace(&self, text: &/* Java */ java::lang::String /**/, mut search_string: &/* Java */ java::lang::String /**/, replacement: &/* Java */ java::lang::String /**/, max: i32) -> /* Java */ java::lang::String /**/ {
		if StringUtils::is_empty(text) || StringUtils::is_empty(search_string) || replacement == null || max == 0 {
			return text;
		}
		if self.ignore_case {
			search_string = search_string.toLowerCase();
		}
		let start: i32 = 0;
		let end: i32 = self.index_of(text, search_string, start);
		if end ==  {
			return text;
		}
		/* final */ let repl_length: i32 = search_string.length();
		let increase: i32 = Math::max(replacement.length() - repl_length, 0);
		increase *=  if max < 0 { 16 } else { Math::min(max, 64) };
		/* final */ let buf: StringBuilder = StringBuilder::new(text.length() + increase);
		while end !=  {
			buf.append(text, start, end).append(replacement);
			start = end + repl_length;
			if max -= 1 == 0 {
				break;
			}
			end = self.index_of(text, search_string, start);
		}
		buf.append(text, start, &text.length());
		return buf.toString();
	}

	pub fn replace_once(&self, text: &/* Java */ java::lang::String /**/, search_string: &/* Java */ java::lang::String /**/, replacement: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return self.replace(text, search_string, replacement, 1);
	}

	pub fn starts_with(&self, str: &/* Java */ java::lang::CharSequence /**/, prefix: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if str == null || prefix == null {
			return str == prefix;
		}
		/* final */ let pre_len: i32 = prefix.length();
		if pre_len > str.length() {
			return false;
		}
		return CharSequenceUtils::region_matches(str, self.ignore_case, 0, prefix, 0, pre_len);
	}

	pub fn starts_with_any(&self, sequence: &/* Java */ java::lang::CharSequence /**/, search_strings: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if StringUtils::is_empty(sequence) || ArrayUtils::is_empty(search_strings) {
			return false;
		}
		for /* final */ search_string in search_strings {
			if self.starts_with(sequence, search_string) {
				return true;
			}
		}
		return false;
	}
}

pub struct Builder {
	ignore_case: bool,
	null_is_less: bool,
}

impl Builder {
	fn new() -> org::apache::commons::lang3::strings::Builder {
	// empty
	}

	pub fn get(&self) -> org::apache::commons::lang3::strings::Strings {
		return  if self.ignore_case { CiStrings::new(self.null_is_less) } else { CsStrings::new(self.null_is_less) };
	}

	pub fn set_ignore_case(&mut self, ignore_case: bool) -> org::apache::commons::lang3::strings::Builder {
		self.ignoreCase = ignore_case;
		return self.as_this();
	}

	pub fn set_null_is_less(&mut self, null_is_less: bool) -> org::apache::commons::lang3::strings::Builder {
		self.nullIsLess = null_is_less;
		return self.as_this();
	}
}

impl org::apache::commons::lang3::function::failable_supplier::FailableSupplier for Builder {}

struct CiStrings;

impl CiStrings {
	fn new(null_is_less: bool) -> org::apache::commons::lang3::strings::CiStrings {
		super(true, null_is_less);
	}

	pub fn compare(&self, s1: &/* Java */ java::lang::String /**/, s2: &/* Java */ java::lang::String /**/) -> i32 {
		if s1 == s2 {
			// Both null or same object
			return 0;
		}
		if s1 == null {
			return  if self.is_null_is_less() { -1 } else { 1 };
		}
		if s2 == null {
			return  if self.is_null_is_less() { 1 } else { -1 };
		}
		return s1.compareToIgnoreCase(s2);
	}

	pub fn contains(&self, str: &/* Java */ java::lang::CharSequence /**/, search_str: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if str == null || search_str == null {
			return false;
		}
		/* final */ let len: i32 = search_str.length();
		/* final */ let max: i32 = str.length() - len;
		 {
			let i: i32 = 0;
			while i <= max {
				{
					if CharSequenceUtils::region_matches(str, true, i, search_str, 0, len) {
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return false;
	}

	pub fn equals(&self, cs1: &/* Java */ java::lang::CharSequence /**/, cs2: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if cs1 == cs2 {
			return true;
		}
		if cs1 == null || cs2 == null {
			return false;
		}
		if cs1.length() != cs2.length() {
			return false;
		}
		return CharSequenceUtils::region_matches(cs1, true, 0, cs2, 0, &cs1.length());
	}

	pub fn equals(&self, s1: &/* Java */ java::lang::String /**/, s2: &/* Java */ java::lang::String /**/) -> bool {
		return  if s1 == null { s2 == null } else { s1.equalsIgnoreCase(s2) };
	}

	pub fn index_of(&self, str: &/* Java */ java::lang::CharSequence /**/, search_str: &/* Java */ java::lang::CharSequence /**/, mut start_pos: i32) -> i32 {
		if str == null || search_str == null {
			return ;
		}
		if start_pos < 0 {
			start_pos = 0;
		}
		/* final */ let end_limit: i32 = str.length() - search_str.length() + 1;
		if start_pos > end_limit {
			return ;
		}
		if search_str.length() == 0 {
			return start_pos;
		}
		 {
			let i: i32 = start_pos;
			while i < end_limit {
				{
					if CharSequenceUtils::region_matches(str, true, i, search_str, 0, &search_str.length()) {
						return i;
					}
				}
				i += 1;
			 }
		 }
	
		return ;
	}

	pub fn last_index_of(&self, str: &/* Java */ java::lang::CharSequence /**/, search_str: &/* Java */ java::lang::CharSequence /**/, mut start_pos: i32) -> i32 {
		if str == null || search_str == null {
			return ;
		}
		/* final */ let search_str_length: i32 = search_str.length();
		/* final */ let str_length: i32 = str.length();
		if start_pos > str_length - search_str_length {
			start_pos = str_length - search_str_length;
		}
		if start_pos < 0 {
			return ;
		}
		if search_str_length == 0 {
			return start_pos;
		}
		 {
			let i: i32 = start_pos;
			while i >= 0 {
				{
					if CharSequenceUtils::region_matches(str, true, i, search_str, 0, search_str_length) {
						return i;
					}
				}
				i -= 1;
			 }
		 }
	
		return ;
	}
}

struct CsStrings;

impl CsStrings {
	fn new(null_is_less: bool) -> org::apache::commons::lang3::strings::CsStrings {
		super(false, null_is_less);
	}

	pub fn compare(&self, s1: &/* Java */ java::lang::String /**/, s2: &/* Java */ java::lang::String /**/) -> i32 {
		if s1 == s2 {
			// Both null or same object
			return 0;
		}
		if s1 == null {
			return  if self.is_null_is_less() { -1 } else { 1 };
		}
		if s2 == null {
			return  if self.is_null_is_less() { 1 } else { -1 };
		}
		return s1.compareTo(s2);
	}

	pub fn contains(&self, seq: &/* Java */ java::lang::CharSequence /**/, search_seq: &/* Java */ java::lang::CharSequence /**/) -> bool {
		return CharSequenceUtils::index_of(seq, search_seq, 0) >= 0;
	}

	pub fn equals(&self, cs1: &/* Java */ java::lang::CharSequence /**/, cs2: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if cs1 == cs2 {
			return true;
		}
		if cs1 == null || cs2 == null {
			return false;
		}
		if cs1.length() != cs2.length() {
			return false;
		}
		if cs1 instanceof String && cs2 instanceof String {
			return cs1.equals(cs2);
		}
		// Step-wise comparison
		/* final */ let length: i32 = cs1.length();
		 {
			let i: i32 = 0;
			while i < length {
				{
					if cs1.charAt(i) != cs2.charAt(i) {
						return false;
					}
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn equals(&self, s1: &/* Java */ java::lang::String /**/, s2: &/* Java */ java::lang::String /**/) -> bool {
		return org::apache::commons::lang3::strings::Strings::eq(s1, s2);
	}

	pub fn index_of(&self, seq: &/* Java */ java::lang::CharSequence /**/, search_seq: &/* Java */ java::lang::CharSequence /**/, start_pos: i32) -> i32 {
		return CharSequenceUtils::index_of(seq, search_seq, start_pos);
	}

	pub fn last_index_of(&self, seq: &/* Java */ java::lang::CharSequence /**/, search_seq: &/* Java */ java::lang::CharSequence /**/, start_pos: i32) -> i32 {
		return CharSequenceUtils::last_index_of(seq, search_seq, start_pos);
	}
}