use java::io::Serializable;
use java::util::Iterator;
use java::util::NoSuchElementException;
use java::util::Objects;

struct CharRange {
	start: u16,
	end: u16,
	negated: bool,
	i_to_string: /* Java */ java::lang::String /**/,
}

impl CharRange {
	static serialVersionUID: i64 = 8270183163158333422;

	static EMPTY_ARRAY: &[org::apache::commons::lang3::char_range::CharRange] = ;

	pub fn is(&self, ch: u16) -> org::apache::commons::lang3::char_range::CharRange {
		return CharRange::new(ch, ch, false);
	}

	pub fn is_in(&self, start: u16, end: u16) -> org::apache::commons::lang3::char_range::CharRange {
		return CharRange::new(start, end, false);
	}

	pub fn is_not(&self, ch: u16) -> org::apache::commons::lang3::char_range::CharRange {
		return CharRange::new(ch, ch, true);
	}

	pub fn is_not_in(&self, start: u16, end: u16) -> org::apache::commons::lang3::char_range::CharRange {
		return CharRange::new(start, end, true);
	}

	fn new(mut start: u16, mut end: u16, negated: bool) -> org::apache::commons::lang3::char_range::CharRange {
		if start > end {
			/* final */ let temp: char = start;
			start = end;
			end = temp;
		}
		self.start = start;
		self.end = end;
		self.negated = negated;
	}

	pub fn contains(&self, ch: u16) -> bool {
		return (ch >= self.start && ch <= self.end) != self.negated;
	}

	pub fn contains(&self, range: &org::apache::commons::lang3::char_range::CharRange) -> bool {
		Objects::requireNonNull(range, "range");
		if self.negated {
			if range.negated {
				return self.start >= range.start && self.end <= range.end;
			}
			return range.end < self.start || range.start > self.end;
		}
		if range.negated {
			return self.start == 0 && self.end == Character::MAX_VALUE;
		}
		return self.start <= range.start && self.end >= range.end;
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		if obj == self {
			return true;
		}
		if !(obj instanceof CharRange) {
			return false;
		}
		/* final */ let other: CharRange = obj as CharRange;
		return self.start == other.start && self.end == other.end && self.negated == other.negated;
	}

	pub fn get_end(&self) -> u16 {
		return self.end;
	}

	pub fn get_start(&self) -> u16 {
		return self.start;
	}

	pub fn hash_code(&self) -> i32 {
		return 83 + self.start + 7 * self.end + ( if self.negated { 1 } else { 0 });
	}

	pub fn is_negated(&self) -> bool {
		return self.negated;
	}

	pub fn iterator(&self) -> /* Java */ java::util::Iterator /**/ {
		return CharacterIterator::new(self);
	}

	pub fn to_string(&mut self) -> /* Java */ java::lang::String /**/ {
		if self.i_to_string == null {
			/* final */ let buf: StringBuilder = StringBuilder::new(4);
			if self.is_negated() {
				buf.append('^');
			}
			buf.append(self.start);
			if self.start != self.end {
				buf.append('-');
				buf.append(self.end);
			}
			self.i_to_string = buf.toString();
		}
		return self.i_to_string;
	}
}

impl /* Java */ java::lang::Iterable /**/ for CharRange {}

impl /* Java */ java::io::Serializable /**/ for CharRange {}

struct CharacterIterator {
	current: u16,
	range: org::apache::commons::lang3::char_range::CharRange,
	has_next: bool,
}

impl CharacterIterator {
	fn new(r: &org::apache::commons::lang3::char_range::CharRange) -> org::apache::commons::lang3::char_range::CharacterIterator {
		self.range = r;
		self.has_next = true;
		if self.range.negated {
			if self.range.start == 0 {
				if self.range.end == Character::MAX_VALUE {
					// This range is an empty set
					self.has_next = false;
				} else {
					self.current = (self.range.end + 1) as char;
				}
			} else {
				self.current = 0;
			}
		} else {
			self.current = self.range.start;
		}
	}

	pub fn has_next(&self) -> bool {
		return self.has_next;
	}

	pub fn next(&self) /* thrown(java.util.NoSuchElementException) */ -> /* Java */ java::lang::Character /**/ {
		if !self.has_next {
			return Err(NoSuchElementException::new());
		}
		/* final */ let cur: char = self.current;
		self.prepare_next();
		return Character::valueOf(cur);
	}

	fn prepare_next(&mut self) {
		if self.range.negated {
			if self.current == Character::MAX_VALUE {
				self.has_next = false;
			} else if self.current + 1 == self.range.start {
				if self.range.end == Character::MAX_VALUE {
					self.has_next = false;
				} else {
					self.current = (self.range.end + 1) as char;
				}
			} else {
				self.current = (self.current + 1) as char;
			}
		} else if self.current < self.range.end {
			self.current = (self.current + 1) as char;
		} else {
			self.has_next = false;
		}
	}

	pub fn remove(&self) /* thrown(java.lang.UnsupportedOperationException) */ {
		return Err(UnsupportedOperationException::new());
	}
}

impl /* Java */ java::util::Iterator /**/ for CharacterIterator {}