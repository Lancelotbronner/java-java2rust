use java::io::Serializable;
use java::util::Collections;
use java::util::HashMap;
use java::util::HashSet;
use java::util::Map;
use java::util::Set;
use java::util::stream::Stream;

pub struct CharSet {
	set: /* Java */ java::util::Set /**/ = Collections::synchronizedSet(HashSet<>::new()),
}

impl CharSet {
	static serialVersionUID: i64 = 5947847346149275958;

	pub static EMPTY: org::apache::commons::lang3::char_set::CharSet = CharSet::new(null as String);

	pub static ASCII_ALPHA: org::apache::commons::lang3::char_set::CharSet = CharSet::new("a-zA-Z");

	pub static ASCII_ALPHA_LOWER: org::apache::commons::lang3::char_set::CharSet = CharSet::new("a-z");

	pub static ASCII_ALPHA_UPPER: org::apache::commons::lang3::char_set::CharSet = CharSet::new("A-Z");

	pub static ASCII_NUMERIC: org::apache::commons::lang3::char_set::CharSet = CharSet::new("0-9");

	static COMMON: /* Java */ java::util::Map /**/ = Collections::synchronizedMap(HashMap<>::new());

	init {
	    COMMON.put(null, EMPTY);
	    COMMON.put(StringUtils.EMPTY, EMPTY);
	    COMMON.put("a-zA-Z", ASCII_ALPHA);
	    COMMON.put("A-Za-z", ASCII_ALPHA);
	    COMMON.put("a-z", ASCII_ALPHA_LOWER);
	    COMMON.put("A-Z", ASCII_ALPHA_UPPER);
	    COMMON.put("0-9", ASCII_NUMERIC);
	}

	pub fn get_instance(&self, set_strs: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::char_set::CharSet {
		if set_strs == null {
			return null;
		}
		if set_strs.length == 1 {
			/* final */ let common: CharSet = self.COMMON.get(set_strs[0]);
			if common != null {
				return common;
			}
		}
		return CharSet::new(set_strs);
	}

	fn new(set: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::char_set::CharSet {
		Stream::of(set).forEach(self::add);
	}

	fn add(&self, str: &/* Java */ java::lang::String /**/) {
		if str == null {
			return;
		}
		/* final */ let len: i32 = str.length();
		let pos: i32 = 0;
		while pos < len {
			/* final */ let remainder: i32 = len - pos;
			if remainder >= 4 && str.charAt(pos) == '^' && str.charAt(pos + 2) == '-' {
				// negated range
				self.set.add(&CharRange::is_not_in(&str.charAt(pos + 1), &str.charAt(pos + 3)));
				pos += 4;
			} else if remainder >= 3 && str.charAt(pos + 1) == '-' {
				// range
				self.set.add(&CharRange::is_in(&str.charAt(pos), &str.charAt(pos + 2)));
				pos += 3;
			} else if remainder >= 2 && str.charAt(pos) == '^' {
				// negated char
				self.set.add(&CharRange::is_not(&str.charAt(pos + 1)));
				pos += 2;
			} else {
				// char
				self.set.add(&CharRange::is(&str.charAt(pos)));
				pos += 1;
			}
		}
	}

	pub fn contains(&self, ch: u16) -> bool {
		synchronized (self.set) {
			return self.set.stream().anyMatch(|range|range.contains(ch));
		}
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		if obj == self {
			return true;
		}
		if !(obj instanceof CharSet) {
			return false;
		}
		/* final */ let other: CharSet = obj as CharSet;
		return self.set.equals(other.set);
	}

	fn get_char_ranges(&self) -> &[org::apache::commons::lang3::char_range::CharRange] {
		return self.set.toArray(CharRange::EMPTY_ARRAY);
	}

	pub fn hash_code(&self) -> i32 {
		return 89 + self.set.hashCode();
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.set.toString();
	}
}

impl /* Java */ java::io::Serializable /**/ for CharSet {}