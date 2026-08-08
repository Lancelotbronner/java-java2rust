pub struct Range {
	begin: com::github::javaparser::position::Position,
	end: com::github::javaparser::position::Position,
}

impl Range {
	pub fn new(begin: &com::github::javaparser::position::Position, end: &com::github::javaparser::position::Position) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::range::Range {
		if begin == null {
			return Err(IllegalArgumentException::new("begin can't be null"));
		}
		if end == null {
			return Err(IllegalArgumentException::new("end can't be null"));
		}
		// Force `begin` to be the position that is earliest within the document:
		if begin.is_before(end)? {
			self.begin = begin;
			self.end = end;
		} else {
			self.begin = end;
			self.end = begin;
		}
	}

	pub fn range(&self, begin: &com::github::javaparser::position::Position, end: &com::github::javaparser::position::Position) -> com::github::javaparser::range::Range {
		return Range::new(begin, end);
	}

	pub fn range(&self, begin_line: i32, begin_column: i32, end_line: i32, end_column: i32) -> com::github::javaparser::range::Range {
		return Range::new(Position::new(begin_line, begin_column), Position::new(end_line, end_column));
	}

	pub fn with_begin_column(&self, begin_column: i32) -> com::github::javaparser::range::Range {
		return com::github::javaparser::range::Range::range(&self.begin.with_column(begin_column), self.end);
	}

	pub fn with_begin_line(&self, begin_line: i32) -> com::github::javaparser::range::Range {
		return com::github::javaparser::range::Range::range(&self.begin.with_line(begin_line), self.end);
	}

	pub fn with_end_column(&self, end_column: i32) -> com::github::javaparser::range::Range {
		return com::github::javaparser::range::Range::range(self.begin, &self.end.with_column(end_column));
	}

	pub fn with_end_line(&self, end_line: i32) -> com::github::javaparser::range::Range {
		return com::github::javaparser::range::Range::range(self.begin, &self.end.with_line(end_line));
	}

	pub fn with_begin(&self, begin: &com::github::javaparser::position::Position) -> com::github::javaparser::range::Range {
		return com::github::javaparser::range::Range::range(begin, self.end);
	}

	pub fn with_end(&self, end: &com::github::javaparser::position::Position) -> com::github::javaparser::range::Range {
		return com::github::javaparser::range::Range::range(self.begin, end);
	}

	pub fn contains(&self, other: &com::github::javaparser::range::Range) /* thrown(java.lang.AssertionError) */ -> bool {
		let begin_result: bool = (self.begin.is_before_or_equal(other.begin)?);
		if !begin_result {
			return false;
		}
	
		return self.end.is_after_or_equal(other.end)?;
	}

	pub fn contains(&self, position: &com::github::javaparser::position::Position) -> bool {
		return self.strictly_contains(position) || self.begin.equals(position) || self.end.equals(position);
	}

	pub fn strictly_contains(&self, other: &com::github::javaparser::range::Range) -> bool {
		let begin_result: bool = (self.begin.is_before(other.begin)?);
		let end_result: bool = (self.end.is_after(other.end)?);
		return begin_result && end_result;
	}

	pub fn strictly_contains(&self, position: &com::github::javaparser::position::Position) /* thrown(java.lang.AssertionError) */ -> bool {
		return position.is_after(self.begin)? && position.is_before(self.end)?;
	}

	pub fn overlaps_with(&self, other: &com::github::javaparser::range::Range) -> bool {
		return (self.contains(other.begin) || self.contains(other.end)) || (other.contains(self.begin) || other.contains(self.end));
	}

	pub fn is_before(&self, position: &com::github::javaparser::position::Position) /* thrown(java.lang.AssertionError) */ -> bool {
		return self.end.is_before(position)?;
	}

	pub fn is_before(&self, other: &com::github::javaparser::range::Range) /* thrown(java.lang.AssertionError) */ -> bool {
		return self.end.is_before(other.begin)?;
	}

	pub fn is_after(&self, position: &com::github::javaparser::position::Position) /* thrown(java.lang.AssertionError) */ -> bool {
		return self.begin.is_after(position)?;
	}

	pub fn is_after(&self, other: &com::github::javaparser::range::Range) /* thrown(java.lang.AssertionError) */ -> bool {
		return self.begin.is_after(other.end)?;
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if o == null || self.getClass() != o.getClass() {
			return false;
		}
	
		let range: Range = o as Range;
		return self.begin.equals(range.begin) && self.end.equals(range.end);
	}

	pub fn hash_code(&self) -> i32 {
		return 31 * self.begin.hash_code() + self.end.hash_code();
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.begin + "-" + self.end;
	}

	pub fn get_line_count(&self) -> i32 {
		return self.end.line - self.begin.line + 1;
	}
}