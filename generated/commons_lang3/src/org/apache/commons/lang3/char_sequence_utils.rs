pub struct CharSequenceUtils;

impl CharSequenceUtils {
	static NOT_FOUND: i32 = -1;

	static TO_STRING_LIMIT: i32 = 16;

	fn check_later_than1(&self, cs: &/* Java */ java::lang::CharSequence /**/, search_char: &/* Java */ java::lang::CharSequence /**/, len2: i32, start1: i32) -> bool {
		 {
			let i: i32 = 1; let j: i32 = len2 - 1;
			while i <= j {
				{
					if cs.charAt(start1 + i) != search_char.charAt(i) || cs.charAt(start1 + j) != search_char.charAt(j) {
						return false;
					}
				}
				i += 1;
				j -= 1;
			 }
		 }
	
		return true;
	}

	fn index_of(&self, cs: &/* Java */ java::lang::CharSequence /**/, search_char: &/* Java */ java::lang::CharSequence /**/, start: i32) -> i32 {
		if cs == null || search_char == null {
			return StringUtils::INDEX_NOT_FOUND;
		}
		if cs instanceof String {
			return (cs as String).indexOf(&search_char.toString(), start);
		}
		if cs instanceof StringBuilder {
			return (cs as StringBuilder).indexOf(&search_char.toString(), start);
		}
		if cs instanceof StringBuffer {
			return (cs as StringBuffer).indexOf(&search_char.toString(), start);
		}
		return cs.toString().indexOf(&search_char.toString(), start);
	//        if (cs instanceof String && searchChar instanceof String) {
	//            // TODO: Do we assume searchChar is usually relatively small;
	//            //       If so then calling toString() on it is better than reverting to
	//            //       the green implementation in the else block
	//            return ((String) cs).indexOf((String) searchChar, start);
	//        } else {
	//            // TODO: Implement rather than convert to String
	//            return cs.toString().indexOf(searchChar.toString(), start);
	//        }
	}

	fn index_of(&self, cs: &/* Java */ java::lang::CharSequence /**/, search_char: i32, mut start: i32) -> i32 {
		if cs instanceof String {
			return (cs as String).indexOf(search_char, start);
		}
		/* final */ let sz: i32 = cs.length();
		if start < 0 {
			start = 0;
		}
		if search_char < Character::MIN_SUPPLEMENTARY_CODE_POINT {
			 {
				let i: i32 = start;
				while i < sz {
					{
						if cs.charAt(i) == search_char {
							return i;
						}
					}
					i += 1;
				 }
			 }
	
			return self.NOT_FOUND;
		}
		//supplementary characters (LANG1300)
		if search_char <= Character::MAX_CODE_POINT {
			/* final */ let chars: Vec<char> = Character::toChars(search_char);
			 {
				let i: i32 = start;
				while i < sz - 1 {
					{
						/* final */ let high: char = cs.charAt(i);
						/* final */ let low: char = cs.charAt(i + 1);
						if high == chars[0] && low == chars[1] {
							return i;
						}
					}
					i += 1;
				 }
			 }
	
		}
		return self.NOT_FOUND;
	}

	fn last_index_of(&self, cs: &/* Java */ java::lang::CharSequence /**/, search_char: &/* Java */ java::lang::CharSequence /**/, mut start: i32) -> i32 {
		if search_char == null || cs == null {
			return self.NOT_FOUND;
		}
		if search_char instanceof String {
			if cs instanceof String {
				return (cs as String).lastIndexOf(search_char as String, start);
			}
			if cs instanceof StringBuilder {
				return (cs as StringBuilder).lastIndexOf(search_char as String, start);
			}
			if cs instanceof StringBuffer {
				return (cs as StringBuffer).lastIndexOf(search_char as String, start);
			}
		}
		/* final */ let len1: i32 = cs.length();
		/* final */ let len2: i32 = search_char.length();
		if start > len1 {
			start = len1;
		}
		if start < 0 || len2 > len1 {
			return self.NOT_FOUND;
		}
		if len2 == 0 {
			return start;
		}
		if len2 <= self.TO_STRING_LIMIT {
			if cs instanceof String {
				return (cs as String).lastIndexOf(&search_char.toString(), start);
			}
			if cs instanceof StringBuilder {
				return (cs as StringBuilder).lastIndexOf(&search_char.toString(), start);
			}
			if cs instanceof StringBuffer {
				return (cs as StringBuffer).lastIndexOf(&search_char.toString(), start);
			}
		}
		if start + len2 > len1 {
			start = len1 - len2;
		}
		/* final */ let char0: char = search_char.charAt(0);
		let i: i32 = start;
		while true {
			while cs.charAt(i) != char0 {
				i -= 1;
				if i < 0 {
					return self.NOT_FOUND;
				}
			}
			if org::apache::commons::lang3::char_sequence_utils::CharSequenceUtils::check_later_than1(cs, search_char, len2, i) {
				return i;
			}
			i -= 1;
			if i < 0 {
				return self.NOT_FOUND;
			}
		}
	}

	fn last_index_of(&self, cs: &/* Java */ java::lang::CharSequence /**/, search_char: i32, mut start: i32) -> i32 {
		if cs instanceof String {
			return (cs as String).lastIndexOf(search_char, start);
		}
		/* final */ let sz: i32 = cs.length();
		if start < 0 {
			return self.NOT_FOUND;
		}
		if start >= sz {
			start = sz - 1;
		}
		if search_char < Character::MIN_SUPPLEMENTARY_CODE_POINT {
			 {
				let i: i32 = start;
				while i >= 0 {
					{
						if cs.charAt(i) == search_char {
							return i;
						}
					}
					i -= 1;
				 }
			 }
	
			return self.NOT_FOUND;
		}
		//NOTE - we must do a forward traversal for this to avoid duplicating code points
		if search_char <= Character::MAX_CODE_POINT {
			/* final */ let chars: Vec<char> = Character::toChars(search_char);
			//make sure it's not the last index
			if start == sz - 1 {
				return self.NOT_FOUND;
			}
			 {
				let i: i32 = start;
				while i >= 0 {
					{
						/* final */ let high: char = cs.charAt(i);
						/* final */ let low: char = cs.charAt(i + 1);
						if chars[0] == high && chars[1] == low {
							return i;
						}
					}
					i -= 1;
				 }
			 }
	
		}
		return self.NOT_FOUND;
	}

	fn region_matches(&self, cs: &/* Java */ java::lang::CharSequence /**/, ignore_case: bool, this_start: i32, substring: &/* Java */ java::lang::CharSequence /**/, start: i32, length: i32) -> bool {
		if cs instanceof String && substring instanceof String {
			return (cs as String).regionMatches(ignore_case, this_start, substring as String, start, length);
		}
		let index1: i32 = this_start;
		let index2: i32 = start;
		let tmp_len: i32 = length;
		// Extract these first so we detect NPEs the same as the java.lang.String version
		/* final */ let src_len: i32 = cs.length() - this_start;
		/* final */ let other_len: i32 = substring.length() - start;
		// Check for invalid parameters
		if this_start < 0 || start < 0 || length < 0 {
			return false;
		}
		// Check that the regions are long enough
		if src_len < length || other_len < length {
			return false;
		}
		while tmp_len -= 1 !!!check!!! post decrement > 0 {
			/* final */ let c1: char = cs.charAt(index1 += 1 !!!check!!! post increment);
			/* final */ let c2: char = substring.charAt(index2 += 1 !!!check!!! post increment);
			if c1 == c2 {
				continue;
			}
			if !ignore_case {
				return false;
			}
			// The real same check as in String#regionMatches(boolean, int, String, int, int):
			/* final */ let u1: char = Character::toUpperCase(c1);
			/* final */ let u2: char = Character::toUpperCase(c2);
			if u1 != u2 && Character::toLowerCase(u1) != Character::toLowerCase(u2) {
				return false;
			}
		}
		return true;
	}

	pub fn sub_sequence(&self, cs: &/* Java */ java::lang::CharSequence /**/, start: i32) -> /* Java */ java::lang::CharSequence /**/ {
		return  if cs == null { null } else { cs.subSequence(start, &cs.length()) };
	}

	pub fn to_char_array(&self, source: &/* Java */ java::lang::CharSequence /**/) -> &[u16] {
		/* final */ let len: i32 = StringUtils::length(source);
		if len == 0 {
			return ArrayUtils::EMPTY_CHAR_ARRAY;
		}
		if source instanceof String {
			return (source as String).toCharArray();
		}
		/* final */ let array: [Option<char>; len] = [None; len];
		 {
			let i: i32 = 0;
			while i < len {
				{
					array[i] = source.charAt(i);
				}
				i += 1;
			 }
		 }
	
		return array;
	}

	pub fn new() -> org::apache::commons::lang3::char_sequence_utils::CharSequenceUtils {
	// empty
	}
}