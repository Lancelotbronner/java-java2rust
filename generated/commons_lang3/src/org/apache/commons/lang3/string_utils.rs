use java::io::UnsupportedEncodingException;
use java::nio::CharBuffer;
use java::nio::charset::Charset;
use java::text::Normalizer;
use java::util::ArrayList;
use java::util::Arrays;
use java::util::Iterator;
use java::util::List;
use java::util::Locale;
use java::util::Objects;
use java::util::Set;
use java::util::function::Supplier;
use java::util::regex::Pattern;
use java::util::stream::Collectors;
use crate::org::apache::commons::lang3::function::Suppliers;
use crate::org::apache::commons::lang3::stream::LangCollectors;
use crate::org::apache::commons::lang3::stream::Streams;

pub struct StringUtils;

impl StringUtils {
	static ELLIPSIS3: /* Java */ java::lang::String /**/ = "...";

	pub static SPACE: /* Java */ java::lang::String /**/ = " ";

	pub static EMPTY: /* Java */ java::lang::String /**/ = "";

	static NULL: /* Java */ java::lang::String /**/ = null;

	pub static LF: /* Java */ java::lang::String /**/ = "\n";

	pub static CR: /* Java */ java::lang::String /**/ = "\r";

	pub static INDEX_NOT_FOUND: i32 = -1;

	static PAD_LIMIT: i32 = 8192;

	static DEFAULT_TTL: i32 = 5;

	static STRIP_ACCENTS_PATTERN: /* Java */ java::util::regex::Pattern /**/ = Pattern::compile("\\p{InCombiningDiacriticalMarks}+");

	pub fn abbreviate(&self, str: &/* Java */ java::lang::String /**/, max_width: i32) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::string_utils::StringUtils::abbreviate(str, self.ELLIPSIS3, 0, max_width)?;
	}

	pub fn abbreviate(&self, str: &/* Java */ java::lang::String /**/, offset: i32, max_width: i32) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::string_utils::StringUtils::abbreviate(str, self.ELLIPSIS3, offset, max_width)?;
	}

	pub fn abbreviate(&self, str: &/* Java */ java::lang::String /**/, abbrev_marker: &/* Java */ java::lang::String /**/, max_width: i32) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::string_utils::StringUtils::abbreviate(str, abbrev_marker, 0, max_width)?;
	}

	pub fn abbreviate(&self, str: &/* Java */ java::lang::String /**/, abbrev_marker: &/* Java */ java::lang::String /**/, mut offset: i32, max_width: i32) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		if org::apache::commons::lang3::string_utils::StringUtils::is_not_empty(str) && self.EMPTY.equals(abbrev_marker) && max_width > 0 {
			return org::apache::commons::lang3::string_utils::StringUtils::substring(str, 0, max_width);
		}
		if org::apache::commons::lang3::string_utils::StringUtils::is_any_empty(str, abbrev_marker) {
			return str;
		}
		/* final */ let abbrev_marker_length: i32 = abbrev_marker.length();
		/* final */ let min_abbrev_width: i32 = abbrev_marker_length + 1;
		/* final */ let min_abbrev_width_offset: i32 = abbrev_marker_length + abbrev_marker_length + 1;
		if max_width < min_abbrev_width {
			return Err(IllegalArgumentException::new(&String::format("Minimum abbreviation width is %d", min_abbrev_width)));
		}
		/* final */ let str_len: i32 = str.length();
		if str_len <= max_width {
			return str;
		}
		if offset > str_len {
			offset = str_len;
		}
		if str_len - offset < max_width - abbrev_marker_length {
			offset = str_len - (max_width - abbrev_marker_length);
		}
		if offset <= abbrev_marker_length + 1 {
			return str.substring(0, max_width - abbrev_marker_length) + abbrev_marker;
		}
		if max_width < min_abbrev_width_offset {
			return Err(IllegalArgumentException::new(&String::format("Minimum abbreviation width with offset is %d", min_abbrev_width_offset)));
		}
		if offset + max_width - abbrev_marker_length < str_len {
			return abbrev_marker + org::apache::commons::lang3::string_utils::StringUtils::abbreviate(&str.substring(offset), abbrev_marker, max_width - abbrev_marker_length)?;
		}
		return abbrev_marker + str.substring(str_len - (max_width - abbrev_marker_length));
	}

	pub fn abbreviate_middle(&self, str: &/* Java */ java::lang::String /**/, middle: &/* Java */ java::lang::String /**/, length: i32) -> /* Java */ java::lang::String /**/ {
		if org::apache::commons::lang3::string_utils::StringUtils::is_any_empty(str, middle) || length >= str.length() || length < middle.length() + 2 {
			return str;
		}
		/* final */ let target_sting: i32 = length - middle.length();
		/* final */ let start_offset: i32 = target_sting / 2 + target_sting % 2;
		/* final */ let end_offset: i32 = str.length() - target_sting / 2;
		return str.substring(0, start_offset) + middle + str.substring(end_offset);
	}

	pub fn append_if_missing(&self, str: &/* Java */ java::lang::String /**/, suffix: &/* Java */ java::lang::CharSequence /**/, suffixes: &/* Java */ java::lang::CharSequence /**/) -> /* Java */ java::lang::String /**/ {
		return Strings::org::apache::commons::lang3::strings::Strings::CS.append_if_missing(str, suffix, suffixes);
	}

	pub fn append_if_missing_ignore_case(&self, str: &/* Java */ java::lang::String /**/, suffix: &/* Java */ java::lang::CharSequence /**/, suffixes: &/* Java */ java::lang::CharSequence /**/) -> /* Java */ java::lang::String /**/ {
		return Strings::org::apache::commons::lang3::strings::Strings::CI.append_if_missing(str, suffix, suffixes);
	}

	pub fn capitalize(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(str) {
			return str;
		}
		/* final */ let first_codepoint: i32 = str.codePointAt(0);
		/* final */ let new_code_point: i32 = Character::toTitleCase(first_codepoint);
		if first_codepoint == new_code_point {
			// already capitalized
			return str;
		}
		/* final */ let new_code_points: Vec<i32> = str.codePoints().toArray();
		// copy the first code point
		new_code_points[0] = new_code_point;
		return String::new(new_code_points, 0, new_code_points.length);
	}

	pub fn center(&self, str: &/* Java */ java::lang::String /**/, size: i32) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::string_utils::StringUtils::center(str, size, ' ');
	}

	pub fn center(&self, mut str: &/* Java */ java::lang::String /**/, size: i32, pad_char: u16) -> /* Java */ java::lang::String /**/ {
		if str == null || size <= 0 {
			return str;
		}
		/* final */ let str_len: i32 = str.length();
		/* final */ let pads: i32 = size - str_len;
		if pads <= 0 {
			return str;
		}
		str = org::apache::commons::lang3::string_utils::StringUtils::left_pad(str, str_len + pads / 2, pad_char);
		return org::apache::commons::lang3::string_utils::StringUtils::right_pad(str, size, pad_char);
	}

	pub fn center(&self, mut str: &/* Java */ java::lang::String /**/, size: i32, mut pad_str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if str == null || size <= 0 {
			return str;
		}
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(pad_str) {
			pad_str = self.SPACE;
		}
		/* final */ let str_len: i32 = str.length();
		/* final */ let pads: i32 = size - str_len;
		if pads <= 0 {
			return str;
		}
		str = org::apache::commons::lang3::string_utils::StringUtils::left_pad(str, str_len + pads / 2, pad_str);
		return org::apache::commons::lang3::string_utils::StringUtils::right_pad(str, size, pad_str);
	}

	pub fn chomp(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(str) {
			return str;
		}
		if str.length() == 1 {
			/* final */ let ch: char = str.charAt(0);
			if ch == CharUtils::CR || ch == CharUtils::LF {
				return self.EMPTY;
			}
			return str;
		}
		let last_idx: i32 = str.length() - 1;
		/* final */ let last: char = str.charAt(last_idx);
		if last == CharUtils::LF {
			if str.charAt(last_idx - 1) == CharUtils::CR {
				last_idx -= 1;
			}
		} else if last != CharUtils::CR {
			last_idx += 1;
		}
		return str.substring(0, last_idx);
	}

	pub fn chomp(&self, str: &/* Java */ java::lang::String /**/, separator: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return Strings::org::apache::commons::lang3::strings::Strings::CS.remove_end(str, separator);
	}

	pub fn chop(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if str == null {
			return null;
		}
		/* final */ let str_len: i32 = str.length();
		if str_len < 2 {
			return self.EMPTY;
		}
		/* final */ let last_idx: i32 = str_len - 1;
		/* final */ let ret: String = str.substring(0, last_idx);
		/* final */ let last: char = str.charAt(last_idx);
		if last == CharUtils::LF && ret.charAt(last_idx - 1) == CharUtils::CR {
			return ret.substring(0, last_idx - 1);
		}
		return ret;
	}

	pub fn compare(&self, str1: &/* Java */ java::lang::String /**/, str2: &/* Java */ java::lang::String /**/) -> i32 {
		return Strings::org::apache::commons::lang3::strings::Strings::CS.compare(str1, str2);
	}

	pub fn compare(&self, str1: &/* Java */ java::lang::String /**/, str2: &/* Java */ java::lang::String /**/, null_is_less: bool) -> i32 {
		if str1 == str2 {
			// NOSONARLINT this intentionally uses == to allow for both null
			return 0;
		}
		if str1 == null {
			return  if null_is_less { -1 } else { 1 };
		}
		if str2 == null {
			return  if null_is_less { 1 } else { -1 };
		}
		return str1.compareTo(str2);
	}

	pub fn compare_ignore_case(&self, str1: &/* Java */ java::lang::String /**/, str2: &/* Java */ java::lang::String /**/) -> i32 {
		return Strings::org::apache::commons::lang3::strings::Strings::CI.compare(str1, str2);
	}

	pub fn compare_ignore_case(&self, str1: &/* Java */ java::lang::String /**/, str2: &/* Java */ java::lang::String /**/, null_is_less: bool) -> i32 {
		if str1 == str2 {
			// NOSONARLINT this intentionally uses == to allow for both null
			return 0;
		}
		if str1 == null {
			return  if null_is_less { -1 } else { 1 };
		}
		if str2 == null {
			return  if null_is_less { 1 } else { -1 };
		}
		return str1.compareToIgnoreCase(str2);
	}

	pub fn contains(&self, seq: &/* Java */ java::lang::CharSequence /**/, search_seq: &/* Java */ java::lang::CharSequence /**/) -> bool {
		return Strings::org::apache::commons::lang3::strings::Strings::CS.contains(seq, search_seq);
	}

	pub fn contains(&self, seq: &/* Java */ java::lang::CharSequence /**/, search_char: i32) -> bool {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(seq) {
			return false;
		}
		return CharSequenceUtils::index_of(seq, search_char, 0) >= 0;
	}

	pub fn contains_any(&self, cs: &/* Java */ java::lang::CharSequence /**/, search_chars: u16) -> bool {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(cs) || ArrayUtils.isEmpty(search_chars) {
			return false;
		}
		/* final */ let cs_length: i32 = cs.length();
		/* final */ let search_length: i32 = search_chars.length;
		/* final */ let cs_last: i32 = cs_length - 1;
		/* final */ let search_last: i32 = search_length - 1;
		 {
			let i: i32 = 0;
			while i < cs_length {
				{
					/* final */ let ch: char = cs.charAt(i);
					 {
						let j: i32 = 0;
						while j < search_length {
							{
								if search_chars[j] == ch {
									if !Character::isHighSurrogate(ch) || j == search_last || i < cs_last && search_chars[j + 1] == cs.charAt(i + 1) {
										return true;
									}
								}
							}
							j += 1;
						 }
					 }
	
				}
				i += 1;
			 }
		 }
	
		return false;
	}

	pub fn contains_any(&self, cs: &/* Java */ java::lang::CharSequence /**/, search_chars: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if search_chars == null {
			return false;
		}
		return org::apache::commons::lang3::string_utils::StringUtils::contains_any(cs, &CharSequenceUtils::to_char_array(search_chars));
	}

	pub fn contains_any(&self, cs: &/* Java */ java::lang::CharSequence /**/, search_char_sequences: &/* Java */ java::lang::CharSequence /**/) -> bool {
		return Strings::org::apache::commons::lang3::strings::Strings::CS.contains_any(cs, search_char_sequences);
	}

	pub fn contains_any_ignore_case(&self, cs: &/* Java */ java::lang::CharSequence /**/, search_char_sequences: &/* Java */ java::lang::CharSequence /**/) -> bool {
		return Strings::org::apache::commons::lang3::strings::Strings::CI.contains_any(cs, search_char_sequences);
	}

	pub fn contains_ignore_case(&self, str: &/* Java */ java::lang::CharSequence /**/, search_str: &/* Java */ java::lang::CharSequence /**/) -> bool {
		return Strings::org::apache::commons::lang3::strings::Strings::CI.contains(str, search_str);
	}

	pub fn contains_none(&self, cs: &/* Java */ java::lang::CharSequence /**/, search_chars: u16) -> bool {
		if cs == null || search_chars == null {
			return true;
		}
		/* final */ let cs_len: i32 = cs.length();
		/* final */ let cs_last: i32 = cs_len - 1;
		/* final */ let search_len: i32 = search_chars.length;
		/* final */ let search_last: i32 = search_len - 1;
		 {
			let i: i32 = 0;
			while i < cs_len {
				{
					/* final */ let ch: char = cs.charAt(i);
					 {
						let j: i32 = 0;
						while j < search_len {
							{
								if search_chars[j] == ch {
									if !Character::isHighSurrogate(ch) || j == search_last || i < cs_last && search_chars[j + 1] == cs.charAt(i + 1) {
										return false;
									}
								}
							}
							j += 1;
						 }
					 }
	
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn contains_none(&self, cs: &/* Java */ java::lang::CharSequence /**/, invalid_chars: &/* Java */ java::lang::String /**/) -> bool {
		if invalid_chars == null {
			return true;
		}
		return org::apache::commons::lang3::string_utils::StringUtils::contains_none(cs, &invalid_chars.toCharArray());
	}

	pub fn contains_only(&self, cs: &/* Java */ java::lang::CharSequence /**/, valid: u16) -> bool {
		// All these pre-checks are to maintain API with an older version
		if valid == null || cs == null {
			return false;
		}
		if cs.length() == 0 {
			return true;
		}
		if valid.length == 0 {
			return false;
		}
		return org::apache::commons::lang3::string_utils::StringUtils::index_of_any_but(cs, valid) == self.INDEX_NOT_FOUND;
	}

	pub fn contains_only(&self, cs: &/* Java */ java::lang::CharSequence /**/, valid_chars: &/* Java */ java::lang::String /**/) -> bool {
		if cs == null || valid_chars == null {
			return false;
		}
		return org::apache::commons::lang3::string_utils::StringUtils::contains_only(cs, &valid_chars.toCharArray());
	}

	pub fn contains_whitespace(&self, seq: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(seq) {
			return false;
		}
		/* final */ let str_len: i32 = seq.length();
		 {
			let i: i32 = 0;
			while i < str_len {
				{
					if Character::isWhitespace(&seq.charAt(i)) {
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return false;
	}

	fn convert_remaining_accent_characters(&self, decomposed: &/* Java */ java::lang::StringBuilder /**/) {
		 {
			let i: i32 = 0;
			while i < decomposed.length() {
				{
					/* final */ let char_at: char = decomposed.charAt(i);
					match char_at {
						'\u0141' =>  {
							decomposed.setCharAt(i, 'L');
							break;
						}
						'\u0142' =>  {
							decomposed.setCharAt(i, 'l');
							break;
						}
						// D with stroke
						'\u0110' =>  {
							// LATIN CAPITAL LETTER D WITH STROKE
							decomposed.setCharAt(i, 'D');
							break;
						}
						'\u0111' =>  {
							// LATIN SMALL LETTER D WITH STROKE
							decomposed.setCharAt(i, 'd');
							break;
						}
						// I with bar
						'\u0197' =>  {
							decomposed.setCharAt(i, 'I');
							break;
						}
						'\u0268' =>  {
							decomposed.setCharAt(i, 'i');
							break;
						}
						'\u1D7B' =>  {
							decomposed.setCharAt(i, 'I');
							break;
						}
						'\u1DA4' =>  {
							decomposed.setCharAt(i, 'i');
							break;
						}
						'\u1DA7' =>  {
							decomposed.setCharAt(i, 'I');
							break;
						}
						// U with bar
						'\u0244' =>  {
							// LATIN CAPITAL LETTER U BAR
							decomposed.setCharAt(i, 'U');
							break;
						}
						'\u0289' =>  {
							// LATIN SMALL LETTER U BAR
							decomposed.setCharAt(i, 'u');
							break;
						}
						'\u1D7E' =>  {
							// LATIN SMALL CAPITAL LETTER U WITH STROKE
							decomposed.setCharAt(i, 'U');
							break;
						}
						'\u1DB6' =>  {
							// MODIFIER LETTER SMALL U BAR
							decomposed.setCharAt(i, 'u');
							break;
						}
						// T with stroke
						'\u0166' =>  {
							// LATIN CAPITAL LETTER T WITH STROKE
							decomposed.setCharAt(i, 'T');
							break;
						}
						'\u0167' =>  {
							// LATIN SMALL LETTER T WITH STROKE
							decomposed.setCharAt(i, 't');
							break;
						}
						_ =>  {
							break;
						}
					}
				}
				i += 1;
			 }
		 }
	
	}

	pub fn count_matches(&self, str: &/* Java */ java::lang::CharSequence /**/, ch: u16) -> i32 {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(str) {
			return 0;
		}
		let count: i32 = 0;
		// We could also call str.toCharArray() for faster lookups but that would generate more garbage.
		 {
			let i: i32 = 0;
			while i < str.length() {
				{
					if ch == str.charAt(i) {
						count += 1;
					}
				}
				i += 1;
			 }
		 }
	
		return count;
	}

	pub fn count_matches(&self, str: &/* Java */ java::lang::CharSequence /**/, sub: &/* Java */ java::lang::CharSequence /**/) -> i32 {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(str) || org::apache::commons::lang3::string_utils::StringUtils::is_empty(sub) {
			return 0;
		}
		let count: i32 = 0;
		let idx: i32 = 0;
		while (idx = CharSequenceUtils::index_of(str, sub, idx)) != self.INDEX_NOT_FOUND {
			count += 1;
			idx += sub.length();
		}
		return count;
	}

	pub fn default_if_blank<T: /* Java */ java::lang::CharSequence /**/>(&self, str: &T, default_str: &T) -> T {
		return  if org::apache::commons::lang3::string_utils::StringUtils::is_blank(str) { default_str } else { str };
	}

	pub fn default_if_empty<T: /* Java */ java::lang::CharSequence /**/>(&self, str: &T, default_str: &T) -> T {
		return  if org::apache::commons::lang3::string_utils::StringUtils::is_empty(str) { default_str } else { str };
	}

	pub fn default_string(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return Objects::toString(str, self.EMPTY);
	}

	pub fn default_string(&self, str: &/* Java */ java::lang::String /**/, null_default: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return Objects::toString(str, null_default);
	}

	pub fn delete_whitespace(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(str) {
			return str;
		}
		/* final */ let sz: i32 = str.length();
		/* final */ let chs: [Option<char>; sz] = [None; sz];
		let count: i32 = 0;
		 {
			let i: i32 = 0;
			while i < sz {
				{
					if !Character::isWhitespace(&str.charAt(i)) {
						chs[count += 1 !!!check!!! post increment] = str.charAt(i);
					}
				}
				i += 1;
			 }
		 }
	
		if count == sz {
			return str;
		}
		if count == 0 {
			return self.EMPTY;
		}
		return String::new(chs, 0, count);
	}

	pub fn difference(&self, str1: &/* Java */ java::lang::String /**/, str2: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if str1 == null {
			return str2;
		}
		if str2 == null {
			return str1;
		}
		/* final */ let at: i32 = org::apache::commons::lang3::string_utils::StringUtils::index_of_difference(str1, str2);
		if at == self.INDEX_NOT_FOUND {
			return self.EMPTY;
		}
		return str2.substring(at);
	}

	pub fn ends_with(&self, str: &/* Java */ java::lang::CharSequence /**/, suffix: &/* Java */ java::lang::CharSequence /**/) -> bool {
		return Strings::org::apache::commons::lang3::strings::Strings::CS.ends_with(str, suffix);
	}

	pub fn ends_with_any(&self, sequence: &/* Java */ java::lang::CharSequence /**/, search_strings: &/* Java */ java::lang::CharSequence /**/) -> bool {
		return Strings::org::apache::commons::lang3::strings::Strings::CS.ends_with_any(sequence, search_strings);
	}

	pub fn ends_with_ignore_case(&self, str: &/* Java */ java::lang::CharSequence /**/, suffix: &/* Java */ java::lang::CharSequence /**/) -> bool {
		return Strings::org::apache::commons::lang3::strings::Strings::CI.ends_with(str, suffix);
	}

	pub fn equals(&self, cs1: &/* Java */ java::lang::CharSequence /**/, cs2: &/* Java */ java::lang::CharSequence /**/) -> bool {
		return Strings::org::apache::commons::lang3::strings::Strings::CS.equals(cs1, cs2);
	}

	pub fn equals_any(&self, string: &/* Java */ java::lang::CharSequence /**/, search_strings: &/* Java */ java::lang::CharSequence /**/) -> bool {
		return Strings::org::apache::commons::lang3::strings::Strings::CS.equals_any(string, search_strings);
	}

	pub fn equals_any_ignore_case(&self, string: &/* Java */ java::lang::CharSequence /**/, search_strings: &/* Java */ java::lang::CharSequence /**/) -> bool {
		return Strings::org::apache::commons::lang3::strings::Strings::CI.equals_any(string, search_strings);
	}

	pub fn equals_ignore_case(&self, cs1: &/* Java */ java::lang::CharSequence /**/, cs2: &/* Java */ java::lang::CharSequence /**/) -> bool {
		return Strings::org::apache::commons::lang3::strings::Strings::CI.equals(cs1, cs2);
	}

	pub fn first_non_blank<T: /* Java */ java::lang::CharSequence /**/>(&self, values: &T) -> T {
		if values != null {
			for /* final */ val in values {
				if org::apache::commons::lang3::string_utils::StringUtils::is_not_blank(val) {
					return val;
				}
			}
		}
		return null;
	}

	pub fn first_non_empty<T: /* Java */ java::lang::CharSequence /**/>(&self, values: &T) -> T {
		if values != null {
			for /* final */ val in values {
				if org::apache::commons::lang3::string_utils::StringUtils::is_not_empty(val) {
					return val;
				}
			}
		}
		return null;
	}

	pub fn get_bytes(&self, string: &/* Java */ java::lang::String /**/, charset: &/* Java */ java::nio::charset::Charset /**/) -> &[i8] {
		return  if string == null { ArrayUtils::EMPTY_BYTE_ARRAY } else { string.getBytes(&Charsets::to_charset(charset)) };
	}

	pub fn get_bytes(&self, string: &/* Java */ java::lang::String /**/, charset: &/* Java */ java::lang::String /**/) /* thrown(java.io.UnsupportedEncodingException) */ -> &[i8] {
		return  if string == null { ArrayUtils::EMPTY_BYTE_ARRAY } else { string.getBytes(&Charsets::to_charset_name(charset)) };
	}

	pub fn get_common_prefix(&self, strs: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if ArrayUtils::is_empty(strs) {
			return self.EMPTY;
		}
		/* final */ let smallest_index_of_diff: i32 = org::apache::commons::lang3::string_utils::StringUtils::index_of_difference(strs);
		if smallest_index_of_diff == self.INDEX_NOT_FOUND {
			// all strings were identical
			if strs[0] == null {
				return self.EMPTY;
			}
			return strs[0];
		}
		if smallest_index_of_diff == 0 {
			// there were no common initial characters
			return self.EMPTY;
		}
		// we found a common initial character sequence
		return strs[0].substring(0, smallest_index_of_diff);
	}

	pub fn get_digits(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(str) {
			return str;
		}
		/* final */ let sz: i32 = str.length();
		/* final */ let str_digits: StringBuilder = StringBuilder::new(sz);
		 {
			let i: i32 = 0;
			while i < sz {
				{
					/* final */ let temp_char: char = str.charAt(i);
					if Character::isDigit(temp_char) {
						str_digits.append(temp_char);
					}
				}
				i += 1;
			 }
		 }
	
		return str_digits.toString();
	}

	pub fn get_fuzzy_distance(&self, term: &/* Java */ java::lang::CharSequence /**/, query: &/* Java */ java::lang::CharSequence /**/, locale: &/* Java */ java::util::Locale /**/) /* thrown(java.lang.IllegalArgumentException) */ -> i32 {
		if term == null || query == null {
			return Err(IllegalArgumentException::new("Strings must not be null"));
		}
		if locale == null {
			return Err(IllegalArgumentException::new("Locale must not be null"));
		}
		// fuzzy logic is case-insensitive. We normalize the Strings to lower
		// case right from the start. Turning characters to lower case
		// via Character.toLowerCase(char) is unfortunately insufficient
		// as it does not accept a locale.
		/* final */ let term_lower_case: String = term.toString().toLowerCase(locale);
		/* final */ let query_lower_case: String = query.toString().toLowerCase(locale);
		// the resulting score
		let score: i32 = 0;
		// the position in the term which will be scanned next for potential
		// query character matches
		let term_index: i32 = 0;
		// index of the previously matched character in the term
		let previous_matching_character_index: i32 = Integer::MIN_VALUE;
		 {
			let query_index: i32 = 0;
			while query_index < query_lower_case.length() {
				{
					/* final */ let query_char: char = query_lower_case.charAt(query_index);
					let term_character_match_found: bool = false;
					while term_index < term_lower_case.length() && !term_character_match_found {
						{
							/* final */ let term_char: char = term_lower_case.charAt(term_index);
							if query_char == term_char {
								// simple character matches result in one point
								score += 1;
								// the score.
								if previous_matching_character_index + 1 == term_index {
									score += 2;
								}
								previous_matching_character_index = term_index;
								// we can leave the nested loop. Every character in the
								// query can match at most one character in the term.
								term_character_match_found = true;
							}
						}
						term_index += 1;
					 }
	
				}
				query_index += 1;
			 }
		 }
	
		return score;
	}

	pub fn get_if_blank<T: /* Java */ java::lang::CharSequence /**/>(&self, str: &T, default_supplier: &/* Java */ java::util::function::Supplier /**/) -> T {
		return  if org::apache::commons::lang3::string_utils::StringUtils::is_blank(str) { Suppliers::get(default_supplier) } else { str };
	}

	pub fn get_if_empty<T: /* Java */ java::lang::CharSequence /**/>(&self, str: &T, default_supplier: &/* Java */ java::util::function::Supplier /**/) -> T {
		return  if org::apache::commons::lang3::string_utils::StringUtils::is_empty(str) { Suppliers::get(default_supplier) } else { str };
	}

	pub fn get_jaro_winkler_distance(&self, first: &/* Java */ java::lang::CharSequence /**/, second: &/* Java */ java::lang::CharSequence /**/) /* thrown(java.lang.IllegalArgumentException) */ -> f64 {
		/* final */ const DEFAULT_SCALING_FACTOR: f64 = 0.1;
		if first == null || second == null {
			return Err(IllegalArgumentException::new("Strings must not be null"));
		}
		/* final */ let mtp: Vec<i32> = org::apache::commons::lang3::string_utils::StringUtils::matches(first, second);
		/* final */ let m: f64 = mtp[0];
		if m == 0 {
			return 0D.0;
		}
		/* final */ let j: f64 = (m / first.length() + m / second.length() + (m - mtp[1]) / m) / 3;
		/* final */ let jw: f64 =  if j < 0.7 { j } else { j + Math::min(DEFAULT_SCALING_FACTOR, 1D.0 / mtp[3]) * mtp[2] * (1D.0 - j) };
		return Math::round(jw * 100.0) / 100.0;
	}

	pub fn get_levenshtein_distance(&self, mut s: &/* Java */ java::lang::CharSequence /**/, mut t: &/* Java */ java::lang::CharSequence /**/) /* thrown(java.lang.IllegalArgumentException) */ -> i32 {
		if s == null || t == null {
			return Err(IllegalArgumentException::new("Strings must not be null"));
		}
		let n: i32 = s.length();
		let m: i32 = t.length();
		if n == 0 {
			return m;
		}
		if m == 0 {
			return n;
		}
		if n > m {
			// swap the input strings to consume less memory
			/* final */ let tmp: CharSequence = s;
			s = t;
			t = tmp;
			n = m;
			m = t.length();
		}
		/* final */ let p: [i32; n + 1] = [0; n + 1];
		// indexes into strings s and t
		// iterates through s
		let i: i32;
		// iterates through t
		let j: i32;
		let upperleft: i32;
		let upper: i32;
		// jth character of t
		let j_of_t: char;
		let cost: i32;
		 {
			i = 0;
			while i <= n {
				{
					p[i] = i;
				}
				i += 1;
			 }
		 }
	
		 {
			j = 1;
			while j <= m {
				{
					upperleft = p[0];
					j_of_t = t.charAt(j - 1);
					p[0] = j;
					 {
						i = 1;
						while i <= n {
							{
								upper = p[i];
								cost =  if s.charAt(i - 1) == j_of_t { 0 } else { 1 };
								// minimum of cell to the left+1, to the top+1, diagonally left and up +cost
								p[i] = Math::min(&Math::min(p[i - 1] + 1, p[i] + 1), upperleft + cost);
								upperleft = upper;
							}
							i += 1;
						 }
					 }
	
				}
				j += 1;
			 }
		 }
	
		return p[n];
	}

	pub fn get_levenshtein_distance(&self, mut s: &/* Java */ java::lang::CharSequence /**/, mut t: &/* Java */ java::lang::CharSequence /**/, threshold: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i32 {
		if s == null || t == null {
			return Err(IllegalArgumentException::new("Strings must not be null"));
		}
		if threshold < 0 {
			return Err(IllegalArgumentException::new("Threshold must not be negative"));
		}
		/* 
	        This implementation only computes the distance if it's less than or equal to the
	        threshold value, returning -1 if it's greater.  The advantage is performance: unbounded
	        distance is O(nm), but a bound of k allows us to reduce it to O(km) time by only
	        computing a diagonal stripe of width 2k + 1 of the cost table.
	        It is also possible to use this to compute the unbounded Levenshtein distance by starting
	        the threshold at 1 and doubling each time until the distance is found; this is O(dm), where
	        d is the distance.
	
	        One subtlety comes from needing to ignore entries on the border of our stripe
	        for example,
	        p[] = |#|#|#|*
	        d[] =  *|#|#|#|
	        We must ignore the entry to the left of the leftmost member
	        We must ignore the entry above the rightmost member
	
	        Another subtlety comes from our stripe running off the matrix if the strings aren't
	        of the same size.  Since string s is always swapped to be the shorter of the two,
	        the stripe will always run off to the upper right instead of the lower left of the matrix.
	
	        As a concrete example, suppose s is of length 5, t is of length 7, and our threshold is 1.
	        In this case we're going to walk a stripe of length 3.  The matrix would look like so:
	
	           1 2 3 4 5
	        1 |#|#| | | |
	        2 |#|#|#| | |
	        3 | |#|#|#| |
	        4 | | |#|#|#|
	        5 | | | |#|#|
	        6 | | | | |#|
	        7 | | | | | |
	
	        Note how the stripe leads off the table as there is no possible way to turn a string of length 5
	        into one of length 7 in edit distance of 1.
	
	        Additionally, this implementation decreases memory usage by using two
	        single-dimensional arrays and swapping them back and forth instead of allocating
	        an entire n by m matrix.  This requires a few minor changes, such as immediately returning
	        when it's detected that the stripe has run off the matrix and initially filling the arrays with
	        large values so that entries we don't compute are ignored.
	
	        See Algorithms on Strings, Trees and Sequences by Dan Gusfield for some discussion.
	         */ 
		// length of s
		let n: i32 = s.length();
		// length of t
		let m: i32 = t.length();
		// if one string is empty, the edit distance is necessarily the length of the other
		if n == 0 {
			return  if m <= threshold { m } else { -1 };
		}
		if m == 0 {
			return  if n <= threshold { n } else { -1 };
		}
		if Math::abs(n - m) > threshold {
			// no need to calculate the distance if the length difference is greater than the threshold
			return -1;
		}
		if n > m {
			// swap the two strings to consume less memory
			/* final */ let tmp: CharSequence = s;
			s = t;
			t = tmp;
			n = m;
			m = t.length();
		}
		// 'previous' cost array, horizontally
		let p: [i32; n + 1] = [0; n + 1];
		// cost array, horizontally
		let d: [i32; n + 1] = [0; n + 1];
		// placeholder to assist in swapping p and d
		let tmp: Vec<i32>;
		// fill in starting table values
		/* final */ let boundary: i32 = Math::min(n, threshold) + 1;
		 {
			let i: i32 = 0;
			while i < boundary {
				{
					p[i] = i;
				}
				i += 1;
			 }
		 }
	
		// these fills ensure that the value above the rightmost entry of our
		// stripe will be ignored in following loop iterations
		Arrays::fill(p, boundary, p.length, Integer::MAX_VALUE);
		Arrays::fill(d, Integer::MAX_VALUE);
		// iterates through t
		 {
			let j: i32 = 1;
			while j <= m {
				{
					// jth character of t
					/* final */ let j_of_t: char = t.charAt(j - 1);
					d[0] = j;
					// compute stripe indices, constrain to array size
					/* final */ let min: i32 = Math::max(1, j - threshold);
					/* final */ let max: i32 =  if j > Integer::MAX_VALUE - threshold { n } else { Math::min(n, j + threshold) };
					// the stripe may lead off of the table if s and t are of different sizes
					if min > max {
						return -1;
					}
					// ignore entry left of leftmost
					if min > 1 {
						d[min - 1] = Integer::MAX_VALUE;
					}
					// iterates through [min, max] in s
					 {
						let i: i32 = min;
						while i <= max {
							{
								if s.charAt(i - 1) == j_of_t {
									// diagonally left and up
									d[i] = p[i - 1];
								} else {
									// 1 + minimum of cell to the left, to the top, diagonally left and up
									d[i] = 1 + Math::min(&Math::min(d[i - 1], p[i]), p[i - 1]);
								}
							}
							i += 1;
						 }
					 }
	
					// copy current distance counts to 'previous row' distance counts
					tmp = p;
					p = d;
					d = tmp;
				}
				j += 1;
			 }
		 }
	
		// distance
		if p[n] <= threshold {
			return p[n];
		}
		return -1;
	}

	pub fn index_of(&self, seq: &/* Java */ java::lang::CharSequence /**/, search_seq: &/* Java */ java::lang::CharSequence /**/) -> i32 {
		return Strings::org::apache::commons::lang3::strings::Strings::CS.index_of(seq, search_seq);
	}

	pub fn index_of(&self, seq: &/* Java */ java::lang::CharSequence /**/, search_seq: &/* Java */ java::lang::CharSequence /**/, start_pos: i32) -> i32 {
		return Strings::org::apache::commons::lang3::strings::Strings::CS.index_of(seq, search_seq, start_pos);
	}

	pub fn index_of(&self, seq: &/* Java */ java::lang::CharSequence /**/, search_char: i32) -> i32 {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(seq) {
			return self.INDEX_NOT_FOUND;
		}
		return CharSequenceUtils::index_of(seq, search_char, 0);
	}

	pub fn index_of(&self, seq: &/* Java */ java::lang::CharSequence /**/, search_char: i32, start_pos: i32) -> i32 {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(seq) {
			return self.INDEX_NOT_FOUND;
		}
		return CharSequenceUtils::index_of(seq, search_char, start_pos);
	}

	pub fn index_of_any(&self, cs: &/* Java */ java::lang::CharSequence /**/, search_chars: u16) -> i32 {
		return org::apache::commons::lang3::string_utils::StringUtils::index_of_any(cs, 0, search_chars);
	}

	pub fn index_of_any(&self, str: &/* Java */ java::lang::CharSequence /**/, search_strs: &/* Java */ java::lang::CharSequence /**/) -> i32 {
		if str == null || search_strs == null {
			return self.INDEX_NOT_FOUND;
		}
		// String's can't have a MAX_VALUEth index.
		let ret: i32 = Integer::MAX_VALUE;
		let tmp: i32;
		for /* final */ search in search_strs {
			if search == null {
				continue;
			}
			tmp = CharSequenceUtils::index_of(str, search, 0);
			if tmp == self.INDEX_NOT_FOUND {
				continue;
			}
			if tmp < ret {
				ret = tmp;
			}
		}
		return  if ret == Integer::MAX_VALUE { self.INDEX_NOT_FOUND } else { ret };
	}

	pub fn index_of_any(&self, cs: &/* Java */ java::lang::CharSequence /**/, cs_start: i32, search_chars: u16) -> i32 {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(cs) || ArrayUtils.isEmpty(search_chars) {
			return self.INDEX_NOT_FOUND;
		}
		/* final */ let cs_len: i32 = cs.length();
		/* final */ let cs_last: i32 = cs_len - 1;
		/* final */ let search_len: i32 = search_chars.length;
		/* final */ let search_last: i32 = search_len - 1;
		 {
			let i: i32 = cs_start;
			while i < cs_len {
				{
					/* final */ let ch: char = cs.charAt(i);
					 {
						let j: i32 = 0;
						while j < search_len {
							{
								if search_chars[j] == ch {
									// ch is a supplementary character
									if i >= cs_last || j >= search_last || !Character::isHighSurrogate(ch) || search_chars[j + 1] == cs.charAt(i + 1) {
										return i;
									}
								}
							}
							j += 1;
						 }
					 }
	
				}
				i += 1;
			 }
		 }
	
		return self.INDEX_NOT_FOUND;
	}

	pub fn index_of_any(&self, cs: &/* Java */ java::lang::CharSequence /**/, search_chars: &/* Java */ java::lang::String /**/) -> i32 {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(cs) || org::apache::commons::lang3::string_utils::StringUtils::is_empty(search_chars) {
			return self.INDEX_NOT_FOUND;
		}
		return org::apache::commons::lang3::string_utils::StringUtils::index_of_any(cs, &search_chars.toCharArray());
	}

	pub fn index_of_any_but(&self, cs: &/* Java */ java::lang::CharSequence /**/, search_chars: u16) -> i32 {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(cs) || ArrayUtils::is_empty(search_chars) {
			return self.INDEX_NOT_FOUND;
		}
		return org::apache::commons::lang3::string_utils::StringUtils::index_of_any_but(cs, &CharBuffer::wrap(search_chars));
	}

	pub fn index_of_any_but(&self, seq: &/* Java */ java::lang::CharSequence /**/, search_chars: &/* Java */ java::lang::CharSequence /**/) -> i32 {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(seq) || org::apache::commons::lang3::string_utils::StringUtils::is_empty(search_chars) {
			return self.INDEX_NOT_FOUND;
		}
		/* final */ let search_set_code_points: Set<Integer> = search_chars.codePoints().boxed().collect(&Collectors::toSet());
		// advance character index from one interpreted codepoint to the next
		 {
			let cur_seq_char_idx: i32 = 0;
			while cur_seq_char_idx < seq.length(){
				/* final */ let cur_seq_code_point: i32 = Character::codePointAt(seq, cur_seq_char_idx);
				if !search_set_code_points.contains(cur_seq_code_point) {
					return cur_seq_char_idx;
				}
				// skip indices to paired low-surrogates
				cur_seq_char_idx += Character::charCount(cur_seq_code_point);
			}
		 }
	
		return self.INDEX_NOT_FOUND;
	}

	pub fn index_of_difference(&self, css: &/* Java */ java::lang::CharSequence /**/) -> i32 {
		if ArrayUtils::get_length(css) <= 1 {
			return self.INDEX_NOT_FOUND;
		}
		let any_string_null: bool = false;
		let all_strings_null: bool = true;
		/* final */ let array_len: i32 = css.length;
		let shortest_str_len: i32 = Integer::MAX_VALUE;
		let longest_str_len: i32 = 0;
		// the bottom loop.
		for /* final */ cs in css {
			if cs == null {
				any_string_null = true;
				shortest_str_len = 0;
			} else {
				all_strings_null = false;
				shortest_str_len = Math::min(&cs.length(), shortest_str_len);
				longest_str_len = Math::max(&cs.length(), longest_str_len);
			}
		}
		// handle lists containing all nulls or all empty strings
		if all_strings_null || longest_str_len == 0 && !any_string_null {
			return self.INDEX_NOT_FOUND;
		}
		// handle lists containing some nulls or some empty strings
		if shortest_str_len == 0 {
			return 0;
		}
		// find the position with the first difference across all strings
		let first_diff: i32 = -1;
		 {
			let string_pos: i32 = 0;
			while string_pos < shortest_str_len {
				{
					/* final */ let comparison_char: char = css[0].charAt(string_pos);
					 {
						let array_pos: i32 = 1;
						while array_pos < array_len {
							{
								if css[array_pos].charAt(string_pos) != comparison_char {
									first_diff = string_pos;
									break;
								}
							}
							array_pos += 1;
						 }
					 }
	
					if first_diff != -1 {
						break;
					}
				}
				string_pos += 1;
			 }
		 }
	
		if first_diff == -1 && shortest_str_len != longest_str_len {
			// vary, so return the length of the shortest string.
			return shortest_str_len;
		}
		return first_diff;
	}

	pub fn index_of_difference(&self, cs1: &/* Java */ java::lang::CharSequence /**/, cs2: &/* Java */ java::lang::CharSequence /**/) -> i32 {
		if cs1 == cs2 {
			return self.INDEX_NOT_FOUND;
		}
		if cs1 == null || cs2 == null {
			return 0;
		}
		let i: i32;
		 {
			i = 0;
			while i < cs1.length() && i < cs2.length() {
				{
					if cs1.charAt(i) != cs2.charAt(i) {
						break;
					}
				}
				i += 1;
			 }
		 }
	
		if i < cs2.length() || i < cs1.length() {
			return i;
		}
		return self.INDEX_NOT_FOUND;
	}

	pub fn index_of_ignore_case(&self, str: &/* Java */ java::lang::CharSequence /**/, search_str: &/* Java */ java::lang::CharSequence /**/) -> i32 {
		return Strings::org::apache::commons::lang3::strings::Strings::CI.index_of(str, search_str);
	}

	pub fn index_of_ignore_case(&self, str: &/* Java */ java::lang::CharSequence /**/, search_str: &/* Java */ java::lang::CharSequence /**/, start_pos: i32) -> i32 {
		return Strings::org::apache::commons::lang3::strings::Strings::CI.index_of(str, search_str, start_pos);
	}

	pub fn is_all_blank(&self, css: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if ArrayUtils::is_empty(css) {
			return true;
		}
		for /* final */ cs in css {
			if org::apache::commons::lang3::string_utils::StringUtils::is_not_blank(cs) {
				return false;
			}
		}
		return true;
	}

	pub fn is_all_empty(&self, css: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if ArrayUtils::is_empty(css) {
			return true;
		}
		for /* final */ cs in css {
			if org::apache::commons::lang3::string_utils::StringUtils::is_not_empty(cs) {
				return false;
			}
		}
		return true;
	}

	pub fn is_all_lower_case(&self, cs: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(cs) {
			return false;
		}
		/* final */ let sz: i32 = cs.length();
		 {
			let i: i32 = 0;
			while i < sz {
				{
					if !Character::isLowerCase(&cs.charAt(i)) {
						return false;
					}
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn is_all_upper_case(&self, cs: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(cs) {
			return false;
		}
		/* final */ let sz: i32 = cs.length();
		 {
			let i: i32 = 0;
			while i < sz {
				{
					if !Character::isUpperCase(&cs.charAt(i)) {
						return false;
					}
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn is_alpha(&self, cs: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(cs) {
			return false;
		}
		/* final */ let sz: i32 = cs.length();
		 {
			let i: i32 = 0;
			while i < sz {
				{
					if !Character::isLetter(&cs.charAt(i)) {
						return false;
					}
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn is_alphanumeric(&self, cs: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(cs) {
			return false;
		}
		/* final */ let sz: i32 = cs.length();
		 {
			let i: i32 = 0;
			while i < sz {
				{
					if !Character::isLetterOrDigit(&cs.charAt(i)) {
						return false;
					}
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn is_alphanumeric_space(&self, cs: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if cs == null {
			return false;
		}
		/* final */ let sz: i32 = cs.length();
		 {
			let i: i32 = 0;
			while i < sz {
				{
					/* final */ let now_char: char = cs.charAt(i);
					if now_char != ' ' && !Character::isLetterOrDigit(now_char) {
						return false;
					}
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn is_alpha_space(&self, cs: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if cs == null {
			return false;
		}
		/* final */ let sz: i32 = cs.length();
		 {
			let i: i32 = 0;
			while i < sz {
				{
					/* final */ let now_char: char = cs.charAt(i);
					if now_char != ' ' && !Character::isLetter(now_char) {
						return false;
					}
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn is_any_blank(&self, css: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if ArrayUtils::is_empty(css) {
			return false;
		}
		for /* final */ cs in css {
			if org::apache::commons::lang3::string_utils::StringUtils::is_blank(cs) {
				return true;
			}
		}
		return false;
	}

	pub fn is_any_empty(&self, css: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if ArrayUtils::is_empty(css) {
			return false;
		}
		for /* final */ cs in css {
			if org::apache::commons::lang3::string_utils::StringUtils::is_empty(cs) {
				return true;
			}
		}
		return false;
	}

	pub fn is_ascii_printable(&self, cs: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if cs == null {
			return false;
		}
		/* final */ let sz: i32 = cs.length();
		 {
			let i: i32 = 0;
			while i < sz {
				{
					if !CharUtils::is_ascii_printable(&cs.charAt(i)) {
						return false;
					}
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn is_blank(&self, cs: &/* Java */ java::lang::CharSequence /**/) -> bool {
		/* final */ let str_len: i32 = org::apache::commons::lang3::string_utils::StringUtils::length(cs);
		if str_len == 0 {
			return true;
		}
		 {
			let i: i32 = 0;
			while i < str_len {
				{
					if !Character::isWhitespace(&cs.charAt(i)) {
						return false;
					}
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn is_empty(&self, cs: &/* Java */ java::lang::CharSequence /**/) -> bool {
		return cs == null || cs.length() == 0;
	}

	pub fn is_mixed_case(&self, cs: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(cs) || cs.length() == 1 {
			return false;
		}
		let contains_uppercase: bool = false;
		let contains_lowercase: bool = false;
		/* final */ let sz: i32 = cs.length();
		 {
			let i: i32 = 0;
			while i < sz {
				{
					/* final */ let now_char: char = cs.charAt(i);
					if Character::isUpperCase(now_char) {
						contains_uppercase = true;
					} else if Character::isLowerCase(now_char) {
						contains_lowercase = true;
					}
					if contains_uppercase && contains_lowercase {
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return false;
	}

	pub fn is_none_blank(&self, css: &/* Java */ java::lang::CharSequence /**/) -> bool {
		return !org::apache::commons::lang3::string_utils::StringUtils::is_any_blank(css);
	}

	pub fn is_none_empty(&self, css: &/* Java */ java::lang::CharSequence /**/) -> bool {
		return !org::apache::commons::lang3::string_utils::StringUtils::is_any_empty(css);
	}

	pub fn is_not_blank(&self, cs: &/* Java */ java::lang::CharSequence /**/) -> bool {
		return !org::apache::commons::lang3::string_utils::StringUtils::is_blank(cs);
	}

	pub fn is_not_empty(&self, cs: &/* Java */ java::lang::CharSequence /**/) -> bool {
		return !org::apache::commons::lang3::string_utils::StringUtils::is_empty(cs);
	}

	pub fn is_numeric(&self, cs: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(cs) {
			return false;
		}
		/* final */ let sz: i32 = cs.length();
		 {
			let i: i32 = 0;
			while i < sz {
				{
					if !Character::isDigit(&cs.charAt(i)) {
						return false;
					}
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn is_numeric_space(&self, cs: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if cs == null {
			return false;
		}
		/* final */ let sz: i32 = cs.length();
		 {
			let i: i32 = 0;
			while i < sz {
				{
					/* final */ let now_char: char = cs.charAt(i);
					if now_char != ' ' && !Character::isDigit(now_char) {
						return false;
					}
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn is_whitespace(&self, cs: &/* Java */ java::lang::CharSequence /**/) -> bool {
		if cs == null {
			return false;
		}
		/* final */ let sz: i32 = cs.length();
		 {
			let i: i32 = 0;
			while i < sz {
				{
					if !Character::isWhitespace(&cs.charAt(i)) {
						return false;
					}
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn join(&self, array: &&[bool], delimiter: u16) -> /* Java */ java::lang::String /**/ {
		if array == null {
			return null;
		}
		return org::apache::commons::lang3::string_utils::StringUtils::join(array, delimiter, 0, array.length);
	}

	pub fn join(&self, array: &&[bool], delimiter: u16, start_index: i32, end_index: i32) -> /* Java */ java::lang::String /**/ {
		if array == null {
			return null;
		}
		if end_index - start_index <= 0 {
			return self.EMPTY;
		}
		/* final */ let string_builder: StringBuilder = StringBuilder::new(array.length * 5 + array.length - 1);
		 {
			let i: i32 = start_index;
			while i < end_index {
				{
					string_builder.append(array[i]).append(delimiter);
				}
				i += 1;
			 }
		 }
	
		return string_builder.substring(0, string_builder.length() - 1);
	}

	pub fn join(&self, array: &&[i8], delimiter: u16) -> /* Java */ java::lang::String /**/ {
		if array == null {
			return null;
		}
		return .join(array, delimiter, 0, array.length);
	}

	pub fn join(&self, array: &&[i8], delimiter: u16, start_index: i32, end_index: i32) -> /* Java */ java::lang::String /**/ {
		if array == null {
			return null;
		}
		if end_index - start_index <= 0 {
			return self.EMPTY;
		}
		/* final */ let string_builder: StringBuilder = StringBuilder::new();
		 {
			let i: i32 = start_index;
			while i < end_index {
				{
					string_builder.append(array[i]).append(delimiter);
				}
				i += 1;
			 }
		 }
	
		return string_builder.substring(0, string_builder.length() - 1);
	}

	pub fn join(&self, array: &&[u16], delimiter: u16) -> /* Java */ java::lang::String /**/ {
		if array == null {
			return null;
		}
		return .join(array, delimiter, 0, array.length);
	}

	pub fn join(&self, array: &&[u16], delimiter: u16, start_index: i32, end_index: i32) -> /* Java */ java::lang::String /**/ {
		if array == null {
			return null;
		}
		if end_index - start_index <= 0 {
			return self.EMPTY;
		}
		/* final */ let string_builder: StringBuilder = StringBuilder::new(array.length * 2 - 1);
		 {
			let i: i32 = start_index;
			while i < end_index {
				{
					string_builder.append(array[i]).append(delimiter);
				}
				i += 1;
			 }
		 }
	
		return string_builder.substring(0, string_builder.length() - 1);
	}

	pub fn join(&self, array: &&[f64], delimiter: u16) -> /* Java */ java::lang::String /**/ {
		if array == null {
			return null;
		}
		return org::apache::commons::lang3::string_utils::StringUtils::join(array, delimiter, 0, array.length);
	}

	pub fn join(&self, array: &&[f64], delimiter: u16, start_index: i32, end_index: i32) -> /* Java */ java::lang::String /**/ {
		if array == null {
			return null;
		}
		if end_index - start_index <= 0 {
			return self.EMPTY;
		}
		/* final */ let string_builder: StringBuilder = StringBuilder::new();
		 {
			let i: i32 = start_index;
			while i < end_index {
				{
					string_builder.append(array[i]).append(delimiter);
				}
				i += 1;
			 }
		 }
	
		return string_builder.substring(0, string_builder.length() - 1);
	}

	pub fn join(&self, array: &&[f32], delimiter: u16) -> /* Java */ java::lang::String /**/ {
		if array == null {
			return null;
		}
		return org::apache::commons::lang3::string_utils::StringUtils::join(array, delimiter, 0, array.length);
	}

	pub fn join(&self, array: &&[f32], delimiter: u16, start_index: i32, end_index: i32) -> /* Java */ java::lang::String /**/ {
		if array == null {
			return null;
		}
		if end_index - start_index <= 0 {
			return self.EMPTY;
		}
		/* final */ let string_builder: StringBuilder = StringBuilder::new();
		 {
			let i: i32 = start_index;
			while i < end_index {
				{
					string_builder.append(array[i]).append(delimiter);
				}
				i += 1;
			 }
		 }
	
		return string_builder.substring(0, string_builder.length() - 1);
	}

	pub fn join(&self, array: &&[i32], separator: u16) -> /* Java */ java::lang::String /**/ {
		if array == null {
			return null;
		}
		return .join(array, separator, 0, array.length);
	}

	pub fn join(&self, array: &&[i32], delimiter: u16, start_index: i32, end_index: i32) -> /* Java */ java::lang::String /**/ {
		if array == null {
			return null;
		}
		if end_index - start_index <= 0 {
			return self.EMPTY;
		}
		/* final */ let string_builder: StringBuilder = StringBuilder::new();
		 {
			let i: i32 = start_index;
			while i < end_index {
				{
					string_builder.append(array[i]).append(delimiter);
				}
				i += 1;
			 }
		 }
	
		return string_builder.substring(0, string_builder.length() - 1);
	}

	pub fn join(&self, iterable: &/* Java */ java::lang::Iterable /**/, separator: u16) -> /* Java */ java::lang::String /**/ {
		return  if iterable != null { org::apache::commons::lang3::string_utils::StringUtils::join(&iterable.iterator(), separator) } else { null };
	}

	pub fn join(&self, iterable: &/* Java */ java::lang::Iterable /**/, separator: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return  if iterable != null { org::apache::commons::lang3::string_utils::StringUtils::join(&iterable.iterator(), separator) } else { null };
	}

	pub fn join(&self, iterator: &/* Java */ java::util::Iterator /**/, separator: u16) -> /* Java */ java::lang::String /**/ {
		// handle null, zero and one elements before building a buffer
		if iterator == null {
			return null;
		}
		if !iterator.hasNext() {
			return self.EMPTY;
		}
		return Streams::of(iterator).collect(&LangCollectors::joining(&ObjectUtils::to_string(&String::valueOf(separator)), self.EMPTY, self.EMPTY, ObjectUtils::toString));
	}

	pub fn join(&self, iterator: &/* Java */ java::util::Iterator /**/, separator: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		// handle null, zero and one elements before building a buffer
		if iterator == null {
			return null;
		}
		if !iterator.hasNext() {
			return self.EMPTY;
		}
		return Streams::of(iterator).collect(&LangCollectors::joining(&ObjectUtils::to_string(separator), self.EMPTY, self.EMPTY, ObjectUtils::toString));
	}

	pub fn join(&self, list: &/* Java */ java::util::List /**/, separator: u16, start_index: i32, end_index: i32) -> /* Java */ java::lang::String /**/ {
		if list == null {
			return null;
		}
		/* final */ let no_of_items: i32 = end_index - start_index;
		if no_of_items <= 0 {
			return self.EMPTY;
		}
		/* final */ let sub_list: List<?> = list.subList(start_index, end_index);
		return org::apache::commons::lang3::string_utils::StringUtils::join(&sub_list.iterator(), separator);
	}

	pub fn join(&self, list: &/* Java */ java::util::List /**/, separator: &/* Java */ java::lang::String /**/, start_index: i32, end_index: i32) -> /* Java */ java::lang::String /**/ {
		if list == null {
			return null;
		}
		/* final */ let no_of_items: i32 = end_index - start_index;
		if no_of_items <= 0 {
			return self.EMPTY;
		}
		/* final */ let sub_list: List<?> = list.subList(start_index, end_index);
		return org::apache::commons::lang3::string_utils::StringUtils::join(&sub_list.iterator(), separator);
	}

	pub fn join(&self, array: &&[i64], separator: u16) -> /* Java */ java::lang::String /**/ {
		if array == null {
			return null;
		}
		return org::apache::commons::lang3::string_utils::StringUtils::join(array, separator, 0, array.length);
	}

	pub fn join(&self, array: &&[i64], delimiter: u16, start_index: i32, end_index: i32) -> /* Java */ java::lang::String /**/ {
		if array == null {
			return null;
		}
		if end_index - start_index <= 0 {
			return self.EMPTY;
		}
		/* final */ let string_builder: StringBuilder = StringBuilder::new();
		 {
			let i: i32 = start_index;
			while i < end_index {
				{
					string_builder.append(array[i]).append(delimiter);
				}
				i += 1;
			 }
		 }
	
		return string_builder.substring(0, string_builder.length() - 1);
	}

	pub fn join(&self, array: &&[/* Java */ java::lang::Object /**/], delimiter: u16) -> /* Java */ java::lang::String /**/ {
		if array == null {
			return null;
		}
		return org::apache::commons::lang3::string_utils::StringUtils::join(array, delimiter, 0, array.length);
	}

	pub fn join(&self, array: &&[/* Java */ java::lang::Object /**/], delimiter: u16, start_index: i32, end_index: i32) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::string_utils::StringUtils::join(array, &String::valueOf(delimiter), start_index, end_index);
	}

	pub fn join(&self, array: &&[/* Java */ java::lang::Object /**/], delimiter: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return  if array != null { org::apache::commons::lang3::string_utils::StringUtils::join(array, &ObjectUtils::to_string(delimiter), 0, array.length) } else { null };
	}

	pub fn join(&self, array: &&[/* Java */ java::lang::Object /**/], delimiter: &/* Java */ java::lang::String /**/, start_index: i32, end_index: i32) -> /* Java */ java::lang::String /**/ {
		return  if array != null { Streams::of(array).skip(start_index).limit(&Math::max(0, end_index - start_index)).collect(&LangCollectors::joining(delimiter, self.EMPTY, self.EMPTY, ObjectUtils::toString)) } else { null };
	}

	pub fn join(&self, array: &&[i16], delimiter: u16) -> /* Java */ java::lang::String /**/ {
		if array == null {
			return null;
		}
		return org::apache::commons::lang3::string_utils::StringUtils::join(array, delimiter, 0, array.length);
	}

	pub fn join(&self, array: &&[i16], delimiter: u16, start_index: i32, end_index: i32) -> /* Java */ java::lang::String /**/ {
		if array == null {
			return null;
		}
		if end_index - start_index <= 0 {
			return self.EMPTY;
		}
		/* final */ let string_builder: StringBuilder = StringBuilder::new();
		 {
			let i: i32 = start_index;
			while i < end_index {
				{
					string_builder.append(array[i]).append(delimiter);
				}
				i += 1;
			 }
		 }
	
		return string_builder.substring(0, string_builder.length() - 1);
	}

	pub fn join<T>(&self, elements: &T) -> /* Java */ java::lang::String /**/ {
		return .join(elements, null);
	}

	pub fn join_with(&self, delimiter: &/* Java */ java::lang::String /**/, array: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		if array == null {
			return Err(IllegalArgumentException::new("Object varargs must not be null"));
		}
		return org::apache::commons::lang3::string_utils::StringUtils::join(array, delimiter);
	}

	pub fn last_index_of(&self, seq: &/* Java */ java::lang::CharSequence /**/, search_seq: &/* Java */ java::lang::CharSequence /**/) -> i32 {
		return Strings::org::apache::commons::lang3::strings::Strings::CS.last_index_of(seq, search_seq);
	}

	pub fn last_index_of(&self, seq: &/* Java */ java::lang::CharSequence /**/, search_seq: &/* Java */ java::lang::CharSequence /**/, start_pos: i32) -> i32 {
		return Strings::org::apache::commons::lang3::strings::Strings::CS.last_index_of(seq, search_seq, start_pos);
	}

	pub fn last_index_of(&self, seq: &/* Java */ java::lang::CharSequence /**/, search_char: i32) -> i32 {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(seq) {
			return self.INDEX_NOT_FOUND;
		}
		return CharSequenceUtils::last_index_of(seq, search_char, &seq.length());
	}

	pub fn last_index_of(&self, seq: &/* Java */ java::lang::CharSequence /**/, search_char: i32, start_pos: i32) -> i32 {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(seq) {
			return self.INDEX_NOT_FOUND;
		}
		return CharSequenceUtils::last_index_of(seq, search_char, start_pos);
	}

	pub fn last_index_of_any(&self, str: &/* Java */ java::lang::CharSequence /**/, search_strs: &/* Java */ java::lang::CharSequence /**/) -> i32 {
		if str == null || search_strs == null {
			return self.INDEX_NOT_FOUND;
		}
		let ret: i32 = self.INDEX_NOT_FOUND;
		let tmp: i32;
		for /* final */ search in search_strs {
			if search == null {
				continue;
			}
			tmp = CharSequenceUtils::last_index_of(str, search, &str.length());
			if tmp > ret {
				ret = tmp;
			}
		}
		return ret;
	}

	pub fn last_index_of_ignore_case(&self, str: &/* Java */ java::lang::CharSequence /**/, search_str: &/* Java */ java::lang::CharSequence /**/) -> i32 {
		return Strings::org::apache::commons::lang3::strings::Strings::CI.last_index_of(str, search_str);
	}

	pub fn last_index_of_ignore_case(&self, str: &/* Java */ java::lang::CharSequence /**/, search_str: &/* Java */ java::lang::CharSequence /**/, start_pos: i32) -> i32 {
		return Strings::org::apache::commons::lang3::strings::Strings::CI.last_index_of(str, search_str, start_pos);
	}

	pub fn last_ordinal_index_of(&self, str: &/* Java */ java::lang::CharSequence /**/, search_str: &/* Java */ java::lang::CharSequence /**/, ordinal: i32) -> i32 {
		return org::apache::commons::lang3::string_utils::StringUtils::ordinal_index_of(str, search_str, ordinal, true);
	}

	pub fn left(&self, str: &/* Java */ java::lang::String /**/, len: i32) -> /* Java */ java::lang::String /**/ {
		if str == null {
			return null;
		}
		if len < 0 {
			return self.EMPTY;
		}
		if str.length() <= len {
			return str;
		}
		return str.substring(0, len);
	}

	pub fn left_pad(&self, str: &/* Java */ java::lang::String /**/, size: i32) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::string_utils::StringUtils::left_pad(str, size, ' ');
	}

	pub fn left_pad(&self, str: &/* Java */ java::lang::String /**/, size: i32, pad_char: u16) -> /* Java */ java::lang::String /**/ {
		if str == null {
			return null;
		}
		/* final */ let pads: i32 = size - str.length();
		if pads <= 0 {
			// returns original String when possible
			return str;
		}
		if pads > self.PAD_LIMIT {
			return org::apache::commons::lang3::string_utils::StringUtils::left_pad(str, size, &String::valueOf(pad_char));
		}
		return org::apache::commons::lang3::string_utils::StringUtils::repeat(pad_char, pads).concat(str);
	}

	pub fn left_pad(&self, str: &/* Java */ java::lang::String /**/, size: i32, mut pad_str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if str == null {
			return null;
		}
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(pad_str) {
			pad_str = self.SPACE;
		}
		/* final */ let pad_len: i32 = pad_str.length();
		/* final */ let str_len: i32 = str.length();
		/* final */ let pads: i32 = size - str_len;
		if pads <= 0 {
			// returns original String when possible
			return str;
		}
		if pad_len == 1 && pads <= self.PAD_LIMIT {
			return org::apache::commons::lang3::string_utils::StringUtils::left_pad(str, size, &pad_str.charAt(0));
		}
		if pads == pad_len {
			return pad_str.concat(str);
		}
		if pads < pad_len {
			return pad_str.substring(0, pads).concat(str);
		}
		/* final */ let padding: [Option<char>; pads] = [None; pads];
		/* final */ let pad_chars: Vec<char> = pad_str.toCharArray();
		 {
			let i: i32 = 0;
			while i < pads {
				{
					padding[i] = pad_chars[i % pad_len];
				}
				i += 1;
			 }
		 }
	
		return String::new(padding).concat(str);
	}

	pub fn length(&self, cs: &/* Java */ java::lang::CharSequence /**/) -> i32 {
		return  if cs == null { 0 } else { cs.length() };
	}

	pub fn lower_case(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if str == null {
			return null;
		}
		return str.toLowerCase();
	}

	pub fn lower_case(&self, str: &/* Java */ java::lang::String /**/, locale: &/* Java */ java::util::Locale /**/) -> /* Java */ java::lang::String /**/ {
		if str == null {
			return null;
		}
		return str.toLowerCase(&LocaleUtils::to_locale(locale));
	}

	fn matches(&self, first: &/* Java */ java::lang::CharSequence /**/, second: &/* Java */ java::lang::CharSequence /**/) -> &[i32] {
		/* final */ let max: CharSequence;
		/* final */ let min: CharSequence;
		if first.length() > second.length() {
			max = first;
			min = second;
		} else {
			max = second;
			min = first;
		}
		/* final */ let range: i32 = Math::max(max.length() / 2 - 1, 0);
		/* final */ let match_indexes: Vec<i32> = ArrayFill::fill(: [i32; min.length()] = [0; min.length()], -1);
		/* final */ let match_flags: [bool; max.length()] = [false; max.length()];
		let matches: i32 = 0;
		 {
			let mi: i32 = 0;
			while mi < min.length() {
				{
					/* final */ let c1: char = min.charAt(mi);
					 {
						let xi: i32 = Math::max(mi - range, 0); let xn: i32 = Math::min(mi + range + 1, &max.length());
						while xi < xn {
							{
								if !match_flags[xi] && c1 == max.charAt(xi) {
									match_indexes[mi] = xi;
									match_flags[xi] = true;
									matches += 1;
									break;
								}
							}
							xi += 1;
						 }
					 }
	
				}
				mi += 1;
			 }
		 }
	
		/* final */ let ms1: [Option<char>; matches] = [None; matches];
		/* final */ let ms2: [Option<char>; matches] = [None; matches];
		 {
			let i: i32 = 0; let si: i32 = 0;
			while i < min.length() {
				{
					if match_indexes[i] != -1 {
						ms1[si] = min.charAt(i);
						si += 1;
					}
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0; let si: i32 = 0;
			while i < max.length() {
				{
					if match_flags[i] {
						ms2[si] = max.charAt(i);
						si += 1;
					}
				}
				i += 1;
			 }
		 }
	
		let transpositions: i32 = 0;
		 {
			let mi: i32 = 0;
			while mi < ms1.length {
				{
					if ms1[mi] != ms2[mi] {
						transpositions += 1;
					}
				}
				mi += 1;
			 }
		 }
	
		let prefix: i32 = 0;
		 {
			let mi: i32 = 0;
			while mi < min.length() {
				{
					if first.charAt(mi) != second.charAt(mi) {
						break;
					}
					prefix += 1;
				}
				mi += 1;
			 }
		 }
	
		return : [i32; ] = [0; ];
	}

	pub fn mid(&self, str: &/* Java */ java::lang::String /**/, mut pos: i32, len: i32) -> /* Java */ java::lang::String /**/ {
		if str == null {
			return null;
		}
		if len < 0 || pos > str.length() {
			return self.EMPTY;
		}
		if pos < 0 {
			pos = 0;
		}
		if str.length() <= pos + len {
			return str.substring(pos);
		}
		return str.substring(pos, pos + len);
	}

	pub fn normalize_space(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		// See https://github.com/librucha/commons-lang-normalizespaces-benchmark for performance test
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(str) {
			return str;
		}
		/* final */ let size: i32 = str.length();
		/* final */ let new_chars: [Option<char>; size] = [None; size];
		let count: i32 = 0;
		let whitespaces_count: i32 = 0;
		let start_whitespaces: bool = true;
		 {
			let i: i32 = 0;
			while i < size {
				{
					/* final */ let actual_char: char = str.charAt(i);
					/* final */ let is_whitespace: bool = Character::isWhitespace(actual_char);
					if is_whitespace {
						if whitespaces_count == 0 && !start_whitespaces {
							new_chars[count += 1 !!!check!!! post increment] = self.SPACE.charAt(0);
						}
						whitespaces_count += 1;
					} else {
						start_whitespaces = false;
						new_chars[count += 1 !!!check!!! post increment] =  if actual_char == 160 { 32 } else { actual_char };
						whitespaces_count = 0;
					}
				}
				i += 1;
			 }
		 }
	
		if start_whitespaces {
			return self.EMPTY;
		}
		return String::new(new_chars, 0, count - ( if whitespaces_count > 0 { 1 } else { 0 })).trim();
	}

	pub fn ordinal_index_of(&self, str: &/* Java */ java::lang::CharSequence /**/, search_str: &/* Java */ java::lang::CharSequence /**/, ordinal: i32) -> i32 {
		return org::apache::commons::lang3::string_utils::StringUtils::ordinal_index_of(str, search_str, ordinal, false);
	}

	fn ordinal_index_of(&self, str: &/* Java */ java::lang::CharSequence /**/, search_str: &/* Java */ java::lang::CharSequence /**/, ordinal: i32, last_index: bool) -> i32 {
		if str == null || search_str == null || ordinal <= 0 {
			return self.INDEX_NOT_FOUND;
		}
		if search_str.length() == 0 {
			return  if last_index { str.length() } else { 0 };
		}
		let found: i32 = 0;
		// set the initial index beyond the end of the string
		// this is to allow for the initial index decrement/increment
		let index: i32 =  if last_index { str.length() } else { self.INDEX_NOT_FOUND };
		loop { {
			if last_index {
				// step backwards through string
				index = CharSequenceUtils::last_index_of(str, search_str, index - 1);
			} else {
				// step forwards through string
				index = CharSequenceUtils::index_of(str, search_str, index + 1);
			}
			if index < 0 {
				return index;
			}
			found += 1;
		}if !(found < ordinal) break;}
		return index;
	}

	pub fn overlay(&self, str: &/* Java */ java::lang::String /**/, mut overlay: &/* Java */ java::lang::String /**/, mut start: i32, mut end: i32) -> /* Java */ java::lang::String /**/ {
		if str == null {
			return null;
		}
		if overlay == null {
			overlay = self.EMPTY;
		}
		/* final */ let len: i32 = str.length();
		if start < 0 {
			start = 0;
		}
		if start > len {
			start = len;
		}
		if end < 0 {
			end = 0;
		}
		if end > len {
			end = len;
		}
		if start > end {
			/* final */ let temp: i32 = start;
			start = end;
			end = temp;
		}
		return str.substring(0, start) + overlay + str.substring(end);
	}

	pub fn prepend_if_missing(&self, str: &/* Java */ java::lang::String /**/, prefix: &/* Java */ java::lang::CharSequence /**/, prefixes: &/* Java */ java::lang::CharSequence /**/) -> /* Java */ java::lang::String /**/ {
		return Strings::org::apache::commons::lang3::strings::Strings::CS.prepend_if_missing(str, prefix, prefixes);
	}

	pub fn prepend_if_missing_ignore_case(&self, str: &/* Java */ java::lang::String /**/, prefix: &/* Java */ java::lang::CharSequence /**/, prefixes: &/* Java */ java::lang::CharSequence /**/) -> /* Java */ java::lang::String /**/ {
		return Strings::org::apache::commons::lang3::strings::Strings::CI.prepend_if_missing(str, prefix, prefixes);
	}

	pub fn remove(&self, str: &/* Java */ java::lang::String /**/, remove: u16) -> /* Java */ java::lang::String /**/ {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(str) || str.indexOf(remove) == self.INDEX_NOT_FOUND {
			return str;
		}
		/* final */ let chars: Vec<char> = str.toCharArray();
		let pos: i32 = 0;
		 {
			let i: i32 = 0;
			while i < chars.length {
				{
					if chars[i] != remove {
						chars[pos += 1 !!!check!!! post increment] = chars[i];
					}
				}
				i += 1;
			 }
		 }
	
		return String::new(chars, 0, pos);
	}

	pub fn remove(&self, str: &/* Java */ java::lang::String /**/, remove: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return Strings::org::apache::commons::lang3::strings::Strings::CS.remove(str, remove);
	}

	pub fn remove_all(&self, text: &/* Java */ java::lang::String /**/, regex: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return RegExUtils::remove_all(text, regex);
	}

	pub fn remove_end(&self, str: &/* Java */ java::lang::String /**/, remove: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return Strings::org::apache::commons::lang3::strings::Strings::CS.remove_end(str, remove);
	}

	pub fn remove_end_ignore_case(&self, str: &/* Java */ java::lang::String /**/, remove: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return Strings::org::apache::commons::lang3::strings::Strings::CI.remove_end(str, remove);
	}

	pub fn remove_first(&self, text: &/* Java */ java::lang::String /**/, regex: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::string_utils::StringUtils::replace_first(text, regex, self.EMPTY);
	}

	pub fn remove_ignore_case(&self, str: &/* Java */ java::lang::String /**/, remove: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return Strings::org::apache::commons::lang3::strings::Strings::CI.remove(str, remove);
	}

	pub fn remove_pattern(&self, source: &/* Java */ java::lang::String /**/, regex: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return RegExUtils::remove_pattern(source, regex);
	}

	pub fn remove_start(&self, str: &/* Java */ java::lang::String /**/, remove: u16) -> /* Java */ java::lang::String /**/ {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(str) {
			return str;
		}
		return  if str.charAt(0) == remove { str.substring(1) } else { str };
	}

	pub fn remove_start(&self, str: &/* Java */ java::lang::String /**/, remove: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return Strings::org::apache::commons::lang3::strings::Strings::CS.remove_start(str, remove);
	}

	pub fn remove_start_ignore_case(&self, str: &/* Java */ java::lang::String /**/, remove: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return Strings::org::apache::commons::lang3::strings::Strings::CI.remove_start(str, remove);
	}

	pub fn repeat(&self, repeat: u16, count: i32) -> /* Java */ java::lang::String /**/ {
		if count <= 0 {
			return self.EMPTY;
		}
		return String::new(&ArrayFill::fill(: [Option<char>; count] = [None; count], repeat));
	}

	pub fn repeat(&self, repeat: &/* Java */ java::lang::String /**/, count: i32) -> /* Java */ java::lang::String /**/ {
		// Performance tuned for 2.0 (JDK1.4)
		if repeat == null {
			return null;
		}
		if count <= 0 {
			return self.EMPTY;
		}
		/* final */ let input_length: i32 = repeat.length();
		if count == 1 || input_length == 0 {
			return repeat;
		}
		if input_length == 1 && count <= self.PAD_LIMIT {
			return org::apache::commons::lang3::string_utils::StringUtils::repeat(&repeat.charAt(0), count);
		}
		/* final */ let output_length: i32 = input_length * count;
		match input_length {
			1 =>  {
				return org::apache::commons::lang3::string_utils::StringUtils::repeat(&repeat.charAt(0), count);
			}
			2 =>  {
				/* final */ let ch0: char = repeat.charAt(0);
				/* final */ let ch1: char = repeat.charAt(1);
				/* final */ let output2: [Option<char>; output_length] = [None; output_length];
				 {
					let i: i32 = count * 2 - 2;
					while i >= 0 {
						{
							output2[i] = ch0;
							output2[i + 1] = ch1;
						}
						i -= 1;
						i -= 1;
					 }
				 }
	
				return String::new(output2);
			}
			_ =>  {
				/* final */ let buf: StringBuilder = StringBuilder::new(output_length);
				 {
					let i: i32 = 0;
					while i < count {
						{
							buf.append(repeat);
						}
						i += 1;
					 }
				 }
	
				return buf.toString();
			}
		}
	}

	pub fn repeat(&self, repeat: &/* Java */ java::lang::String /**/, separator: &/* Java */ java::lang::String /**/, count: i32) -> /* Java */ java::lang::String /**/ {
		if repeat == null || separator == null {
			return org::apache::commons::lang3::string_utils::StringUtils::repeat(repeat, count);
		}
		// given that repeat(String, int) is quite optimized, better to rely on it than try and splice this into it
		/* final */ let result: String = org::apache::commons::lang3::string_utils::StringUtils::repeat(repeat + separator, count);
		return Strings::org::apache::commons::lang3::strings::Strings::CS.remove_end(result, separator);
	}

	pub fn replace(&self, text: &/* Java */ java::lang::String /**/, search_string: &/* Java */ java::lang::String /**/, replacement: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return Strings::org::apache::commons::lang3::strings::Strings::CS.replace(text, search_string, replacement);
	}

	pub fn replace(&self, text: &/* Java */ java::lang::String /**/, search_string: &/* Java */ java::lang::String /**/, replacement: &/* Java */ java::lang::String /**/, max: i32) -> /* Java */ java::lang::String /**/ {
		return Strings::org::apache::commons::lang3::strings::Strings::CS.replace(text, search_string, replacement, max);
	}

	pub fn replace_all(&self, text: &/* Java */ java::lang::String /**/, regex: &/* Java */ java::lang::String /**/, replacement: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return RegExUtils::replace_all(text, regex, replacement);
	}

	pub fn replace_chars(&self, str: &/* Java */ java::lang::String /**/, search_char: u16, replace_char: u16) -> /* Java */ java::lang::String /**/ {
		if str == null {
			return null;
		}
		return str.replace(search_char, replace_char);
	}

	pub fn replace_chars(&self, str: &/* Java */ java::lang::String /**/, search_chars: &/* Java */ java::lang::String /**/, mut replace_chars: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(str) || org::apache::commons::lang3::string_utils::StringUtils::is_empty(search_chars) {
			return str;
		}
		replace_chars = ObjectUtils::to_string(replace_chars);
		let modified: bool = false;
		/* final */ let replace_chars_length: i32 = replace_chars.length();
		/* final */ let str_length: i32 = str.length();
		/* final */ let buf: StringBuilder = StringBuilder::new(str_length);
		 {
			let i: i32 = 0;
			while i < str_length {
				{
					/* final */ let ch: char = str.charAt(i);
					/* final */ let index: i32 = search_chars.indexOf(ch);
					if index >= 0 {
						modified = true;
						if index < replace_chars_length {
							buf.append(&replace_chars.charAt(index));
						}
					} else {
						buf.append(ch);
					}
				}
				i += 1;
			 }
		 }
	
		if modified {
			return buf.toString();
		}
		return str;
	}

	pub fn replace_each(&self, text: &/* Java */ java::lang::String /**/, search_list: &&[/* Java */ java::lang::String /**/], replacement_list: &&[/* Java */ java::lang::String /**/]) /* thrown(java.lang.IllegalArgumentException | java.lang.IllegalStateException) */ -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::string_utils::StringUtils::replace_each(text, search_list, replacement_list, false, 0)?;
	}

	fn replace_each(&self, text: &/* Java */ java::lang::String /**/, search_list: &&[/* Java */ java::lang::String /**/], replacement_list: &&[/* Java */ java::lang::String /**/], repeat: bool, time_to_live: i32) /* thrown(java.lang.IllegalArgumentException | java.lang.IllegalStateException) */ -> /* Java */ java::lang::String /**/ {
		// let me know if there are performance requests, we can create a harness to measure
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(text) || ArrayUtils::is_empty(search_list) || ArrayUtils::is_empty(replacement_list) {
			return text;
		}
		// if recursing, this shouldn't be less than 0
		if time_to_live < 0 {
			return Err(IllegalStateException::new("Aborting to protect against StackOverflowError - " + "output of one loop is the input of another"));
		}
		/* final */ let search_length: i32 = search_list.length;
		/* final */ let replacement_length: i32 = replacement_list.length;
		// make sure lengths are ok, these need to be equal
		if search_length != replacement_length {
			return Err(IllegalArgumentException::new("Search and Replace array lengths don't match: " + search_length + " vs " + replacement_length));
		}
		// keep track of which still have matches
		/* final */ let no_more_matches_for_repl_index: [bool; search_length] = [false; search_length];
		// index on index that the match was found
		let text_index: i32 = -1;
		let replace_index: i32 = -1;
		let temp_index: i32;
		// NOTE: logic duplicated below START
		 {
			let i: i32 = 0;
			while i < search_length {
				{
					if no_more_matches_for_repl_index[i] || org::apache::commons::lang3::string_utils::StringUtils::is_empty(search_list[i]) || replacement_list[i] == null {
						continue;
					}
					temp_index = text.indexOf(search_list[i]);
					// see if we need to keep searching for this
					if temp_index == -1 {
						no_more_matches_for_repl_index[i] = true;
					} else if text_index == -1 || temp_index < text_index {
						text_index = temp_index;
						replace_index = i;
					}
				}
				i += 1;
			 }
		 }
	
		// no search strings found, we are done
		if text_index == -1 {
			return text;
		}
		let start: i32 = 0;
		// get a good guess on the size of the result buffer so it doesn't have to double if it goes over a bit
		let increase: i32 = 0;
		// count the replacement text elements that are larger than their corresponding text being replaced
		 {
			let i: i32 = 0;
			while i < search_list.length {
				{
					if search_list[i] == null || replacement_list[i] == null {
						continue;
					}
					/* final */ let greater: i32 = replacement_list[i].length() - search_list[i].length();
					if greater > 0 {
						// assume 3 matches
						increase += 3 * greater;
					}
				}
				i += 1;
			 }
		 }
	
		// have upper-bound at 20% increase, then let Java take over
		increase = Math::min(increase, text.length() / 5);
		/* final */ let buf: StringBuilder = StringBuilder::new(text.length() + increase);
		while text_index != -1 {
			 {
				let i: i32 = start;
				while i < text_index {
					{
						buf.append(&text.charAt(i));
					}
					i += 1;
				 }
			 }
	
			buf.append(replacement_list[replace_index]);
			start = text_index + search_list[replace_index].length();
			text_index = -1;
			replace_index = -1;
			// NOTE: logic mostly duplicated above START
			 {
				let i: i32 = 0;
				while i < search_length {
					{
						if no_more_matches_for_repl_index[i] || org::apache::commons::lang3::string_utils::StringUtils::is_empty(search_list[i]) || replacement_list[i] == null {
							continue;
						}
						temp_index = text.indexOf(search_list[i], start);
						// see if we need to keep searching for this
						if temp_index == -1 {
							no_more_matches_for_repl_index[i] = true;
						} else if text_index == -1 || temp_index < text_index {
							text_index = temp_index;
							replace_index = i;
						}
					}
					i += 1;
				 }
			 }
	
		// NOTE: logic duplicated above END
		}
		/* final */ let text_length: i32 = text.length();
		 {
			let i: i32 = start;
			while i < text_length {
				{
					buf.append(&text.charAt(i));
				}
				i += 1;
			 }
		 }
	
		/* final */ let result: String = buf.toString();
		if !repeat {
			return result;
		}
		return org::apache::commons::lang3::string_utils::StringUtils::replace_each(result, search_list, replacement_list, repeat, time_to_live - 1)?;
	}

	pub fn replace_each_repeatedly(&self, text: &/* Java */ java::lang::String /**/, search_list: &&[/* Java */ java::lang::String /**/], replacement_list: &&[/* Java */ java::lang::String /**/]) /* thrown(java.lang.IllegalArgumentException | java.lang.IllegalStateException) */ -> /* Java */ java::lang::String /**/ {
		/* final */ let time_to_live: i32 = Math::max(&ArrayUtils::get_length(search_list), self.DEFAULT_TTL);
		return org::apache::commons::lang3::string_utils::StringUtils::replace_each(text, search_list, replacement_list, true, time_to_live)?;
	}

	pub fn replace_first(&self, text: &/* Java */ java::lang::String /**/, regex: &/* Java */ java::lang::String /**/, replacement: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return RegExUtils::replace_first(text, regex, replacement);
	}

	pub fn replace_ignore_case(&self, text: &/* Java */ java::lang::String /**/, search_string: &/* Java */ java::lang::String /**/, replacement: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return Strings::org::apache::commons::lang3::strings::Strings::CI.replace(text, search_string, replacement);
	}

	pub fn replace_ignore_case(&self, text: &/* Java */ java::lang::String /**/, search_string: &/* Java */ java::lang::String /**/, replacement: &/* Java */ java::lang::String /**/, max: i32) -> /* Java */ java::lang::String /**/ {
		return Strings::org::apache::commons::lang3::strings::Strings::CI.replace(text, search_string, replacement, max);
	}

	pub fn replace_once(&self, text: &/* Java */ java::lang::String /**/, search_string: &/* Java */ java::lang::String /**/, replacement: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return Strings::org::apache::commons::lang3::strings::Strings::CS.replace_once(text, search_string, replacement);
	}

	pub fn replace_once_ignore_case(&self, text: &/* Java */ java::lang::String /**/, search_string: &/* Java */ java::lang::String /**/, replacement: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return Strings::org::apache::commons::lang3::strings::Strings::CI.replace_once(text, search_string, replacement);
	}

	pub fn replace_pattern(&self, source: &/* Java */ java::lang::String /**/, regex: &/* Java */ java::lang::String /**/, replacement: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return RegExUtils::replace_pattern(source, regex, replacement);
	}

	pub fn reverse(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if str == null {
			return null;
		}
		return StringBuilder::new(str).reverse().toString();
	}

	pub fn reverse_delimited(&self, str: &/* Java */ java::lang::String /**/, separator_char: u16) -> /* Java */ java::lang::String /**/ {
		/* final */ let strs: Vec<String> = org::apache::commons::lang3::string_utils::StringUtils::split(str, separator_char);
		ArrayUtils::reverse(strs);
		return org::apache::commons::lang3::string_utils::StringUtils::join(strs, separator_char);
	}

	pub fn right(&self, str: &/* Java */ java::lang::String /**/, len: i32) -> /* Java */ java::lang::String /**/ {
		if str == null {
			return null;
		}
		if len < 0 {
			return self.EMPTY;
		}
		if str.length() <= len {
			return str;
		}
		return str.substring(str.length() - len);
	}

	pub fn right_pad(&self, str: &/* Java */ java::lang::String /**/, size: i32) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::string_utils::StringUtils::right_pad(str, size, ' ');
	}

	pub fn right_pad(&self, str: &/* Java */ java::lang::String /**/, size: i32, pad_char: u16) -> /* Java */ java::lang::String /**/ {
		if str == null {
			return null;
		}
		/* final */ let pads: i32 = size - str.length();
		if pads <= 0 {
			// returns original String when possible
			return str;
		}
		if pads > self.PAD_LIMIT {
			return org::apache::commons::lang3::string_utils::StringUtils::right_pad(str, size, &String::valueOf(pad_char));
		}
		return str.concat(&org::apache::commons::lang3::string_utils::StringUtils::repeat(pad_char, pads));
	}

	pub fn right_pad(&self, str: &/* Java */ java::lang::String /**/, size: i32, mut pad_str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if str == null {
			return null;
		}
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(pad_str) {
			pad_str = self.SPACE;
		}
		/* final */ let pad_len: i32 = pad_str.length();
		/* final */ let str_len: i32 = str.length();
		/* final */ let pads: i32 = size - str_len;
		if pads <= 0 {
			// returns original String when possible
			return str;
		}
		if pad_len == 1 && pads <= self.PAD_LIMIT {
			return org::apache::commons::lang3::string_utils::StringUtils::right_pad(str, size, &pad_str.charAt(0));
		}
		if pads == pad_len {
			return str.concat(pad_str);
		}
		if pads < pad_len {
			return str.concat(&pad_str.substring(0, pads));
		}
		/* final */ let padding: [Option<char>; pads] = [None; pads];
		/* final */ let pad_chars: Vec<char> = pad_str.toCharArray();
		 {
			let i: i32 = 0;
			while i < pads {
				{
					padding[i] = pad_chars[i % pad_len];
				}
				i += 1;
			 }
		 }
	
		return str.concat(String::new(padding));
	}

	pub fn rotate(&self, str: &/* Java */ java::lang::String /**/, shift: i32) -> /* Java */ java::lang::String /**/ {
		if str == null {
			return null;
		}
		/* final */ let str_len: i32 = str.length();
		if shift == 0 || str_len == 0 || shift % str_len == 0 {
			return str;
		}
		/* final */ let builder: StringBuilder = StringBuilder::new(str_len);
		/* final */ let offset: i32 = -(shift % str_len);
		builder.append(&org::apache::commons::lang3::string_utils::StringUtils::substring(str, offset));
		builder.append(&org::apache::commons::lang3::string_utils::StringUtils::substring(str, 0, offset));
		return builder.toString();
	}

	pub fn split(&self, str: &/* Java */ java::lang::String /**/) -> &[/* Java */ java::lang::String /**/] {
		return org::apache::commons::lang3::string_utils::StringUtils::split(str, null, -1);
	}

	pub fn split(&self, str: &/* Java */ java::lang::String /**/, separator_char: u16) -> &[/* Java */ java::lang::String /**/] {
		return org::apache::commons::lang3::string_utils::StringUtils::split_worker(str, separator_char, false);
	}

	pub fn split(&self, str: &/* Java */ java::lang::String /**/, separator_chars: &/* Java */ java::lang::String /**/) -> &[/* Java */ java::lang::String /**/] {
		return org::apache::commons::lang3::string_utils::StringUtils::split_worker(str, separator_chars, -1, false);
	}

	pub fn split(&self, str: &/* Java */ java::lang::String /**/, separator_chars: &/* Java */ java::lang::String /**/, max: i32) -> &[/* Java */ java::lang::String /**/] {
		return org::apache::commons::lang3::string_utils::StringUtils::split_worker(str, separator_chars, max, false);
	}

	pub fn split_by_character_type(&self, str: &/* Java */ java::lang::String /**/) -> &[/* Java */ java::lang::String /**/] {
		return org::apache::commons::lang3::string_utils::StringUtils::split_by_character_type(str, false);
	}

	fn split_by_character_type(&self, str: &/* Java */ java::lang::String /**/, camel_case: bool) -> &[/* Java */ java::lang::String /**/] {
		if str == null {
			return null;
		}
		if str.isEmpty() {
			return ArrayUtils::EMPTY_STRING_ARRAY;
		}
		/* final */ let c: Vec<char> = str.toCharArray();
		/* final */ let list: List<String> = ArrayList<>::new();
		let token_start: i32 = 0;
		let current_type: i32 = Character::getType(c[token_start]);
		 {
			let pos: i32 = token_start + 1;
			while pos < c.length {
				{
					/* final */ let type: i32 = Character::getType(c[pos]);
					if type == current_type {
						continue;
					}
					if camel_case && type == Character::LOWERCASE_LETTER && current_type == Character::UPPERCASE_LETTER {
						/* final */ let new_token_start: i32 = pos - 1;
						if new_token_start != token_start {
							list.add(String::new(c, token_start, new_token_start - token_start));
							token_start = new_token_start;
						}
					} else {
						list.add(String::new(c, token_start, pos - token_start));
						token_start = pos;
					}
					current_type = type;
				}
				pos += 1;
			 }
		 }
	
		list.add(String::new(c, token_start, c.length - token_start));
		return list.toArray(ArrayUtils::EMPTY_STRING_ARRAY);
	}

	pub fn split_by_character_type_camel_case(&self, str: &/* Java */ java::lang::String /**/) -> &[/* Java */ java::lang::String /**/] {
		return org::apache::commons::lang3::string_utils::StringUtils::split_by_character_type(str, true);
	}

	pub fn split_by_whole_separator(&self, str: &/* Java */ java::lang::String /**/, separator: &/* Java */ java::lang::String /**/) -> &[/* Java */ java::lang::String /**/] {
		return org::apache::commons::lang3::string_utils::StringUtils::split_by_whole_separator_worker(str, separator, -1, false);
	}

	pub fn split_by_whole_separator(&self, str: &/* Java */ java::lang::String /**/, separator: &/* Java */ java::lang::String /**/, max: i32) -> &[/* Java */ java::lang::String /**/] {
		return org::apache::commons::lang3::string_utils::StringUtils::split_by_whole_separator_worker(str, separator, max, false);
	}

	pub fn split_by_whole_separator_preserve_all_tokens(&self, str: &/* Java */ java::lang::String /**/, separator: &/* Java */ java::lang::String /**/) -> &[/* Java */ java::lang::String /**/] {
		return org::apache::commons::lang3::string_utils::StringUtils::split_by_whole_separator_worker(str, separator, -1, true);
	}

	pub fn split_by_whole_separator_preserve_all_tokens(&self, str: &/* Java */ java::lang::String /**/, separator: &/* Java */ java::lang::String /**/, max: i32) -> &[/* Java */ java::lang::String /**/] {
		return org::apache::commons::lang3::string_utils::StringUtils::split_by_whole_separator_worker(str, separator, max, true);
	}

	fn split_by_whole_separator_worker(&self, str: &/* Java */ java::lang::String /**/, separator: &/* Java */ java::lang::String /**/, max: i32, preserve_all_tokens: bool) -> &[/* Java */ java::lang::String /**/] {
		if str == null {
			return null;
		}
		/* final */ let len: i32 = str.length();
		if len == 0 {
			return ArrayUtils::EMPTY_STRING_ARRAY;
		}
		if separator == null || self.EMPTY.equals(separator) {
			// Split on whitespace.
			return org::apache::commons::lang3::string_utils::StringUtils::split_worker(str, null, max, preserve_all_tokens);
		}
		/* final */ let separator_length: i32 = separator.length();
		/* final */ let substrings: ArrayList<String> = ArrayList<>::new();
		let number_of_substrings: i32 = 0;
		let beg: i32 = 0;
		let end: i32 = 0;
		while end < len {
			end = str.indexOf(separator, beg);
			if end > -1 {
				if end > beg {
					number_of_substrings += 1;
					if number_of_substrings == max {
						end = len;
						substrings.add(&str.substring(beg));
					} else {
						// The following is OK, because String.substring( beg, end ) excludes
						// the character at the position 'end'.
						substrings.add(&str.substring(beg, end));
						// Set the starting point for the next search.
						// The following is equivalent to beg = end + (separatorLength - 1) + 1,
						// which is the right calculation:
						beg = end + separator_length;
					}
				} else {
					// We found a consecutive occurrence of the separator, so skip it.
					if preserve_all_tokens {
						number_of_substrings += 1;
						if number_of_substrings == max {
							end = len;
							substrings.add(&str.substring(beg));
						} else {
							substrings.add(self.EMPTY);
						}
					}
					beg = end + separator_length;
				}
			} else {
				// String.substring( beg ) goes from 'beg' to the end of the String.
				substrings.add(&str.substring(beg));
				end = len;
			}
		}
		return substrings.toArray(ArrayUtils::EMPTY_STRING_ARRAY);
	}

	pub fn split_preserve_all_tokens(&self, str: &/* Java */ java::lang::String /**/) -> &[/* Java */ java::lang::String /**/] {
		return org::apache::commons::lang3::string_utils::StringUtils::split_worker(str, null, -1, true);
	}

	pub fn split_preserve_all_tokens(&self, str: &/* Java */ java::lang::String /**/, separator_char: u16) -> &[/* Java */ java::lang::String /**/] {
		return org::apache::commons::lang3::string_utils::StringUtils::split_worker(str, separator_char, true);
	}

	pub fn split_preserve_all_tokens(&self, str: &/* Java */ java::lang::String /**/, separator_chars: &/* Java */ java::lang::String /**/) -> &[/* Java */ java::lang::String /**/] {
		return org::apache::commons::lang3::string_utils::StringUtils::split_worker(str, separator_chars, -1, true);
	}

	pub fn split_preserve_all_tokens(&self, str: &/* Java */ java::lang::String /**/, separator_chars: &/* Java */ java::lang::String /**/, max: i32) -> &[/* Java */ java::lang::String /**/] {
		return org::apache::commons::lang3::string_utils::StringUtils::split_worker(str, separator_chars, max, true);
	}

	fn split_worker(&self, str: &/* Java */ java::lang::String /**/, separator_char: u16, preserve_all_tokens: bool) -> &[/* Java */ java::lang::String /**/] {
		// Performance tuned for 2.0 (JDK1.4)
		if str == null {
			return null;
		}
		/* final */ let len: i32 = str.length();
		if len == 0 {
			return ArrayUtils::EMPTY_STRING_ARRAY;
		}
		/* final */ let list: List<String> = ArrayList<>::new();
		let i: i32 = 0;
		let start: i32 = 0;
		let match: bool = false;
		let last_match: bool = false;
		while i < len {
			if str.charAt(i) == separator_char {
				if match || preserve_all_tokens {
					list.add(&str.substring(start, i));
					match = false;
					last_match = true;
				}
				start = i += 1;
				continue;
			}
			last_match = false;
			match = true;
			i += 1;
		}
		if match || preserve_all_tokens && last_match {
			list.add(&str.substring(start, i));
		}
		return list.toArray(ArrayUtils::EMPTY_STRING_ARRAY);
	}

	fn split_worker(&self, str: &/* Java */ java::lang::String /**/, separator_chars: &/* Java */ java::lang::String /**/, max: i32, preserve_all_tokens: bool) -> &[/* Java */ java::lang::String /**/] {
		// Also, StringTokenizer uses isSpace() not isWhitespace()
		if str == null {
			return null;
		}
		/* final */ let len: i32 = str.length();
		if len == 0 {
			return ArrayUtils::EMPTY_STRING_ARRAY;
		}
		/* final */ let list: List<String> = ArrayList<>::new();
		let size_plus1: i32 = 1;
		let i: i32 = 0;
		let start: i32 = 0;
		let match: bool = false;
		let last_match: bool = false;
		if separator_chars == null {
			// Null separator means use whitespace
			while i < len {
				if Character::isWhitespace(&str.charAt(i)) {
					if match || preserve_all_tokens {
						last_match = true;
						if size_plus1 += 1 !!!check!!! post increment == max {
							i = len;
							last_match = false;
						}
						list.add(&str.substring(start, i));
						match = false;
					}
					start = i += 1;
					continue;
				}
				last_match = false;
				match = true;
				i += 1;
			}
		} else if separator_chars.length() == 1 {
			// Optimize 1 character case
			/* final */ let sep: char = separator_chars.charAt(0);
			while i < len {
				if str.charAt(i) == sep {
					if match || preserve_all_tokens {
						last_match = true;
						if size_plus1 += 1 !!!check!!! post increment == max {
							i = len;
							last_match = false;
						}
						list.add(&str.substring(start, i));
						match = false;
					}
					start = i += 1;
					continue;
				}
				last_match = false;
				match = true;
				i += 1;
			}
		} else {
			// standard case
			while i < len {
				if separator_chars.indexOf(&str.charAt(i)) >= 0 {
					if match || preserve_all_tokens {
						last_match = true;
						if size_plus1 += 1 !!!check!!! post increment == max {
							i = len;
							last_match = false;
						}
						list.add(&str.substring(start, i));
						match = false;
					}
					start = i += 1;
					continue;
				}
				last_match = false;
				match = true;
				i += 1;
			}
		}
		if match || preserve_all_tokens && last_match {
			list.add(&str.substring(start, i));
		}
		return list.toArray(ArrayUtils::EMPTY_STRING_ARRAY);
	}

	pub fn starts_with(&self, str: &/* Java */ java::lang::CharSequence /**/, prefix: &/* Java */ java::lang::CharSequence /**/) -> bool {
		return Strings::org::apache::commons::lang3::strings::Strings::CS.starts_with(str, prefix);
	}

	pub fn starts_with_any(&self, sequence: &/* Java */ java::lang::CharSequence /**/, search_strings: &/* Java */ java::lang::CharSequence /**/) -> bool {
		return Strings::org::apache::commons::lang3::strings::Strings::CS.starts_with_any(sequence, search_strings);
	}

	pub fn starts_with_ignore_case(&self, str: &/* Java */ java::lang::CharSequence /**/, prefix: &/* Java */ java::lang::CharSequence /**/) -> bool {
		return Strings::org::apache::commons::lang3::strings::Strings::CI.starts_with(str, prefix);
	}

	pub fn strip(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::string_utils::StringUtils::strip(str, null);
	}

	pub fn strip(&self, mut str: &/* Java */ java::lang::String /**/, strip_chars: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		str = org::apache::commons::lang3::string_utils::StringUtils::strip_start(str, strip_chars);
		return org::apache::commons::lang3::string_utils::StringUtils::strip_end(str, strip_chars);
	}

	pub fn strip_accents(&self, input: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(input) {
			return input;
		}
		/* final */ let decomposed: StringBuilder = StringBuilder::new(&Normalizer::normalize(input, Normalizer::Form::NFKD));
		org::apache::commons::lang3::string_utils::StringUtils::convert_remaining_accent_characters(decomposed);
		return self.STRIP_ACCENTS_PATTERN.matcher(decomposed).replaceAll(self.EMPTY);
	}

	pub fn strip_all(&self, strs: &/* Java */ java::lang::String /**/) -> &[/* Java */ java::lang::String /**/] {
		return .stripAll(strs, null);
	}

	pub fn strip_all(&self, strs: &&[/* Java */ java::lang::String /**/], strip_chars: &/* Java */ java::lang::String /**/) -> &[/* Java */ java::lang::String /**/] {
		/* final */ let strs_len: i32 = ArrayUtils::get_length(strs);
		if strs_len == 0 {
			return strs;
		}
		return ArrayUtils::set_all(: [Option<String>; strs_len] = [None; strs_len], |i|org::apache::commons::lang3::string_utils::StringUtils::strip(strs[i], strip_chars));
	}

	pub fn strip_end(&self, str: &/* Java */ java::lang::String /**/, strip_chars: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		let end: i32 = org::apache::commons::lang3::string_utils::StringUtils::length(str);
		if end == 0 {
			return str;
		}
		if strip_chars == null {
			while end != 0 && Character::isWhitespace(&str.charAt(end - 1)) {
				end -= 1;
			}
		} else if strip_chars.isEmpty() {
			return str;
		} else {
			while end != 0 && strip_chars.indexOf(&str.charAt(end - 1)) != self.INDEX_NOT_FOUND {
				end -= 1;
			}
		}
		return str.substring(0, end);
	}

	pub fn strip_start(&self, str: &/* Java */ java::lang::String /**/, strip_chars: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		/* final */ let str_len: i32 = org::apache::commons::lang3::string_utils::StringUtils::length(str);
		if str_len == 0 {
			return str;
		}
		let start: i32 = 0;
		if strip_chars == null {
			while start != str_len && Character::isWhitespace(&str.charAt(start)) {
				start += 1;
			}
		} else if strip_chars.isEmpty() {
			return str;
		} else {
			while start != str_len && strip_chars.indexOf(&str.charAt(start)) != self.INDEX_NOT_FOUND {
				start += 1;
			}
		}
		return str.substring(start);
	}

	pub fn strip_to_empty(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return  if str == null { self.EMPTY } else { org::apache::commons::lang3::string_utils::StringUtils::strip(str, null) };
	}

	pub fn strip_to_null(&self, mut str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if str == null {
			return null;
		}
		str = org::apache::commons::lang3::string_utils::StringUtils::strip(str, null);
		// NOSONARLINT str cannot be null here
		return  if str.isEmpty() { null } else { str };
	}

	pub fn substring(&self, str: &/* Java */ java::lang::String /**/, mut start: i32) -> /* Java */ java::lang::String /**/ {
		if str == null {
			return null;
		}
		// handle negatives, which means last n characters
		if start < 0 {
			// remember start is negative
			start = str.length() + start;
		}
		if start < 0 {
			start = 0;
		}
		if start > str.length() {
			return self.EMPTY;
		}
		return str.substring(start);
	}

	pub fn substring(&self, str: &/* Java */ java::lang::String /**/, mut start: i32, mut end: i32) -> /* Java */ java::lang::String /**/ {
		if str == null {
			return null;
		}
		// handle negatives
		if end < 0 {
			// remember end is negative
			end = str.length() + end;
		}
		if start < 0 {
			// remember start is negative
			start = str.length() + start;
		}
		// check length next
		if end > str.length() {
			end = str.length();
		}
		// if start is greater than end, return ""
		if start > end {
			return self.EMPTY;
		}
		if start < 0 {
			start = 0;
		}
		if end < 0 {
			end = 0;
		}
		return str.substring(start, end);
	}

	pub fn substring_after(&self, str: &/* Java */ java::lang::String /**/, find: i32) -> /* Java */ java::lang::String /**/ {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(str) {
			return str;
		}
		/* final */ let pos: i32 = str.indexOf(find);
		if pos == self.INDEX_NOT_FOUND {
			return self.EMPTY;
		}
		return str.substring(pos + 1);
	}

	pub fn substring_after(&self, str: &/* Java */ java::lang::String /**/, find: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(str) {
			return str;
		}
		if find == null {
			return self.EMPTY;
		}
		/* final */ let pos: i32 = str.indexOf(find);
		if pos == self.INDEX_NOT_FOUND {
			return self.EMPTY;
		}
		return str.substring(pos + find.length());
	}

	pub fn substring_after_last(&self, str: &/* Java */ java::lang::String /**/, find: i32) -> /* Java */ java::lang::String /**/ {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(str) {
			return str;
		}
		/* final */ let pos: i32 = str.lastIndexOf(find);
		if pos == self.INDEX_NOT_FOUND || pos == str.length() - 1 {
			return self.EMPTY;
		}
		return str.substring(pos + 1);
	}

	pub fn substring_after_last(&self, str: &/* Java */ java::lang::String /**/, find: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(str) {
			return str;
		}
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(find) {
			return self.EMPTY;
		}
		/* final */ let pos: i32 = str.lastIndexOf(find);
		if pos == self.INDEX_NOT_FOUND || pos == str.length() - find.length() {
			return self.EMPTY;
		}
		return str.substring(pos + find.length());
	}

	pub fn substring_before(&self, str: &/* Java */ java::lang::String /**/, find: i32) -> /* Java */ java::lang::String /**/ {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(str) {
			return str;
		}
		/* final */ let pos: i32 = str.indexOf(find);
		if pos == self.INDEX_NOT_FOUND {
			return str;
		}
		return str.substring(0, pos);
	}

	pub fn substring_before(&self, str: &/* Java */ java::lang::String /**/, find: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(str) || find == null {
			return str;
		}
		if find.isEmpty() {
			return self.EMPTY;
		}
		/* final */ let pos: i32 = str.indexOf(find);
		if pos == self.INDEX_NOT_FOUND {
			return str;
		}
		return str.substring(0, pos);
	}

	pub fn substring_before_last(&self, str: &/* Java */ java::lang::String /**/, find: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(str) || org::apache::commons::lang3::string_utils::StringUtils::is_empty(find) {
			return str;
		}
		/* final */ let pos: i32 = str.lastIndexOf(find);
		if pos == self.INDEX_NOT_FOUND {
			return str;
		}
		return str.substring(0, pos);
	}

	pub fn substring_between(&self, str: &/* Java */ java::lang::String /**/, tag: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::string_utils::StringUtils::substring_between(str, tag, tag);
	}

	pub fn substring_between(&self, str: &/* Java */ java::lang::String /**/, open: &/* Java */ java::lang::String /**/, close: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if !ObjectUtils::all_not_null(str, open, close) {
			return null;
		}
		/* final */ let start: i32 = str.indexOf(open);
		if start != self.INDEX_NOT_FOUND {
			/* final */ let end: i32 = str.indexOf(close, start + open.length());
			if end != self.INDEX_NOT_FOUND {
				return str.substring(start + open.length(), end);
			}
		}
		return null;
	}

	pub fn substrings_between(&self, str: &/* Java */ java::lang::String /**/, open: &/* Java */ java::lang::String /**/, close: &/* Java */ java::lang::String /**/) -> &[/* Java */ java::lang::String /**/] {
		if str == null || org::apache::commons::lang3::string_utils::StringUtils::is_empty(open) || org::apache::commons::lang3::string_utils::StringUtils::is_empty(close) {
			return null;
		}
		/* final */ let str_len: i32 = str.length();
		if str_len == 0 {
			return ArrayUtils::EMPTY_STRING_ARRAY;
		}
		/* final */ let close_len: i32 = close.length();
		/* final */ let open_len: i32 = open.length();
		/* final */ let list: List<String> = ArrayList<>::new();
		let pos: i32 = 0;
		while pos < str_len - close_len {
			let start: i32 = str.indexOf(open, pos);
			if start < 0 {
				break;
			}
			start += open_len;
			/* final */ let end: i32 = str.indexOf(close, start);
			if end < 0 {
				break;
			}
			list.add(&str.substring(start, end));
			pos = end + close_len;
		}
		if list.isEmpty() {
			return null;
		}
		return list.toArray(ArrayUtils::EMPTY_STRING_ARRAY);
	}

	pub fn swap_case(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(str) {
			return str;
		}
		/* final */ let str_len: i32 = str.length();
		// cannot be longer than the char array
		/* final */ let new_code_points: [i32; str_len] = [0; str_len];
		let out_offset: i32 = 0;
		 {
			let i: i32 = 0;
			while i < str_len{
				/* final */ let old_codepoint: i32 = str.codePointAt(i);
				/* final */ let new_code_point: i32;
				if Character::isUpperCase(old_codepoint) || Character::isTitleCase(old_codepoint) {
					new_code_point = Character::toLowerCase(old_codepoint);
				} else if Character::isLowerCase(old_codepoint) {
					new_code_point = Character::toUpperCase(old_codepoint);
				} else {
					new_code_point = old_codepoint;
				}
				new_code_points[out_offset += 1 !!!check!!! post increment] = new_code_point;
				i += Character::charCount(new_code_point);
			}
		 }
	
		return String::new(new_code_points, 0, out_offset);
	}

	pub fn to_code_points(&self, cs: &/* Java */ java::lang::CharSequence /**/) -> &[i32] {
		if cs == null {
			return null;
		}
		if cs.length() == 0 {
			return ArrayUtils::EMPTY_INT_ARRAY;
		}
		return cs.toString().codePoints().toArray();
	}

	pub fn to_encoded_string(&self, bytes: &&[i8], charset: &/* Java */ java::nio::charset::Charset /**/) -> /* Java */ java::lang::String /**/ {
		return String::new(bytes, &Charsets::to_charset(charset));
	}

	pub fn to_root_lower_case(&self, source: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return  if source == null { null } else { source.toLowerCase(Locale::ROOT) };
	}

	pub fn to_root_upper_case(&self, source: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return  if source == null { null } else { source.toUpperCase(Locale::ROOT) };
	}

	pub fn to_string(&self, bytes: &&[i8], charset_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return String::new(bytes, &Charsets::to_charset(charset_name));
	}

	pub fn trim(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return  if str == null { null } else { str.trim() };
	}

	pub fn trim_to_empty(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return  if str == null { self.EMPTY } else { str.trim() };
	}

	pub fn trim_to_null(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		/* final */ let ts: String = org::apache::commons::lang3::string_utils::StringUtils::trim(str);
		return  if org::apache::commons::lang3::string_utils::StringUtils::is_empty(ts) { null } else { ts };
	}

	pub fn truncate(&self, str: &/* Java */ java::lang::String /**/, max_width: i32) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::string_utils::StringUtils::truncate(str, 0, max_width)?;
	}

	pub fn truncate(&self, str: &/* Java */ java::lang::String /**/, offset: i32, max_width: i32) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		if offset < 0 {
			return Err(IllegalArgumentException::new("offset cannot be negative"));
		}
		if max_width < 0 {
			return Err(IllegalArgumentException::new("maxWidth cannot be negative"));
		}
		if str == null {
			return null;
		}
		if offset > str.length() {
			return self.EMPTY;
		}
		if str.length() > max_width {
			/* final */ let ix: i32 = Math::min(offset + max_width, &str.length());
			return str.substring(offset, ix);
		}
		return str.substring(offset);
	}

	pub fn uncapitalize(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		/* final */ let str_len: i32 = org::apache::commons::lang3::string_utils::StringUtils::length(str);
		if str_len == 0 {
			return str;
		}
		/* final */ let first_code_point: i32 = str.codePointAt(0);
		/* final */ let new_code_point: i32 = Character::toLowerCase(first_code_point);
		if first_code_point == new_code_point {
			// already uncapitalized
			return str;
		}
		/* final */ let new_code_points: Vec<i32> = str.codePoints().toArray();
		// copy the first code point
		new_code_points[0] = new_code_point;
		return String::new(new_code_points, 0, new_code_points.length);
	}

	pub fn unwrap(&self, str: &/* Java */ java::lang::String /**/, wrap_char: u16) -> /* Java */ java::lang::String /**/ {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(str) || wrap_char == CharUtils::NUL || str.length() == 1 {
			return str;
		}
		if str.charAt(0) == wrap_char && str.charAt(str.length() - 1) == wrap_char {
			/* final */ let start_index: i32 = 0;
			/* final */ let end_index: i32 = str.length() - 1;
			return str.substring(start_index + 1, end_index);
		}
		return str;
	}

	pub fn unwrap(&self, str: &/* Java */ java::lang::String /**/, wrap_token: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(str) || org::apache::commons::lang3::string_utils::StringUtils::is_empty(wrap_token) || str.length() < 2 * wrap_token.length() {
			return str;
		}
		if Strings::org::apache::commons::lang3::strings::Strings::CS.starts_with(str, wrap_token) && Strings::org::apache::commons::lang3::strings::Strings::CS.ends_with(str, wrap_token) {
			return str.substring(&wrap_token.length(), &str.lastIndexOf(wrap_token));
		}
		return str;
	}

	pub fn upper_case(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if str == null {
			return null;
		}
		return str.toUpperCase();
	}

	pub fn upper_case(&self, str: &/* Java */ java::lang::String /**/, locale: &/* Java */ java::util::Locale /**/) -> /* Java */ java::lang::String /**/ {
		if str == null {
			return null;
		}
		return str.toUpperCase(&LocaleUtils::to_locale(locale));
	}

	pub fn value_of(&self, value: &&[u16]) -> /* Java */ java::lang::String /**/ {
		return  if value == null { null } else { String::valueOf(value) };
	}

	pub fn wrap(&self, str: &/* Java */ java::lang::String /**/, wrap_with: u16) -> /* Java */ java::lang::String /**/ {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(str) || wrap_with == CharUtils::NUL {
			return str;
		}
		return wrap_with + str + wrap_with;
	}

	pub fn wrap(&self, str: &/* Java */ java::lang::String /**/, wrap_with: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(str) || org::apache::commons::lang3::string_utils::StringUtils::is_empty(wrap_with) {
			return str;
		}
		return wrap_with.concat(str).concat(wrap_with);
	}

	pub fn wrap_if_missing(&self, str: &/* Java */ java::lang::String /**/, wrap_with: u16) -> /* Java */ java::lang::String /**/ {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(str) || wrap_with == CharUtils::NUL {
			return str;
		}
		/* final */ let wrap_start: bool = str.charAt(0) != wrap_with;
		/* final */ let wrap_end: bool = str.charAt(str.length() - 1) != wrap_with;
		if !wrap_start && !wrap_end {
			return str;
		}
		/* final */ let builder: StringBuilder = StringBuilder::new(str.length() + 2);
		if wrap_start {
			builder.append(wrap_with);
		}
		builder.append(str);
		if wrap_end {
			builder.append(wrap_with);
		}
		return builder.toString();
	}

	pub fn wrap_if_missing(&self, str: &/* Java */ java::lang::String /**/, wrap_with: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if org::apache::commons::lang3::string_utils::StringUtils::is_empty(str) || org::apache::commons::lang3::string_utils::StringUtils::is_empty(wrap_with) {
			return str;
		}
		/* final */ let wrap_start: bool = !str.startsWith(wrap_with);
		/* final */ let wrap_end: bool = !str.endsWith(wrap_with);
		if !wrap_start && !wrap_end {
			return str;
		}
		/* final */ let builder: StringBuilder = StringBuilder::new(str.length() + wrap_with.length() + wrap_with.length());
		if wrap_start {
			builder.append(wrap_with);
		}
		builder.append(str);
		if wrap_end {
			builder.append(wrap_with);
		}
		return builder.toString();
	}

	pub fn new() -> org::apache::commons::lang3::string_utils::StringUtils {
	// empty
	}
}