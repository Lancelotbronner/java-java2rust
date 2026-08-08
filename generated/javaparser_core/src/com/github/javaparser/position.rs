use crate::com::github::javaparser::utils::Utils::assertNotNull;
use java::util::Objects;

pub struct Position {
	line: i32,
	column: i32,
}

impl Position {
	pub static FIRST_LINE: i32 = 1;

	pub static FIRST_COLUMN: i32 = 1;

	pub static HOME: com::github::javaparser::position::Position = Position::new(FIRST_LINE, FIRST_COLUMN);

	pub static ABSOLUTE_BEGIN_LINE: i32 = -1;

	pub static ABSOLUTE_END_LINE: i32 = -2;

	pub fn new(line: i32, column: i32) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::position::Position {
		if line < Position::ABSOLUTE_END_LINE {
			return Err(IllegalArgumentException::new("Can't position at line " + line));
		}
		if column < -1 {
			// TODO: Maybe we need an "ABSOLUTE_BEGIN_LINE" and "ABSOLUTE_END_LINE"?
			return Err(IllegalArgumentException::new("Can't position at column " + column));
		}
		self.line = line;
		self.column = column;
	}

	pub fn pos(&self, line: i32, column: i32) -> com::github::javaparser::position::Position {
		return Position::new(line, column);
	}

	pub fn with_column(&self, column: i32) -> com::github::javaparser::position::Position {
		return Position::new(self.line, column);
	}

	pub fn with_line(&self, line: i32) -> com::github::javaparser::position::Position {
		return Position::new(line, self.column);
	}

	pub fn right(&self, characters: i32) -> com::github::javaparser::position::Position {
		return Position::new(self.line, self.column + characters);
	}

	pub fn next_line(&self) -> com::github::javaparser::position::Position {
		return Position::new(self.line + 1, self.FIRST_COLUMN);
	}

	pub fn valid(&self) -> bool {
		return self.ABSOLUTE_END_LINE == self.line || self.ABSOLUTE_BEGIN_LINE == self.line || self.line >= self.FIRST_LINE && self.column >= self.FIRST_COLUMN;
	}

	pub fn invalid(&self) -> bool {
		return !self.valid();
	}

	pub fn or_if_invalid(&self, alternative_position: &com::github::javaparser::position::Position) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::position::Position {
		com::github::javaparser::utils::utils::Utils::assert_not_null(alternative_position)?;
		if self.valid() {
			return self;
		}
		return  if alternative_position.valid() { alternative_position } else { self };
	}

	pub fn is_after(&self, other_position: &com::github::javaparser::position::Position) /* thrown(java.lang.AssertionError) */ -> bool {
		com::github::javaparser::utils::utils::Utils::assert_not_null(other_position)?;
		if self.line == other_position.line {
			return self.column > other_position.column;
		}
		return self.line > other_position.line || other_position.line == Position::ABSOLUTE_BEGIN_LINE;
	}

	pub fn is_after_or_equal(&self, other_position: &com::github::javaparser::position::Position) /* thrown(java.lang.AssertionError) */ -> bool {
		return self.is_after(other_position)? || self.equals(other_position);
	}

	pub fn is_before(&self, other_position: &com::github::javaparser::position::Position) /* thrown(java.lang.AssertionError) */ -> bool {
		com::github::javaparser::utils::utils::Utils::assert_not_null(other_position)?;
		if self.line == other_position.line {
			return self.column < other_position.column;
		}
		return self.line < other_position.line || other_position.line == Position::ABSOLUTE_END_LINE;
	}

	pub fn is_before_or_equal(&self, other_position: &com::github::javaparser::position::Position) /* thrown(java.lang.AssertionError) */ -> bool {
		return self.is_before(other_position)? || self.equals(other_position);
	}

	pub fn compare_to(&self, other_position: &com::github::javaparser::position::Position) /* thrown(java.lang.AssertionError) */ -> i32 {
		com::github::javaparser::utils::utils::Utils::assert_not_null(other_position)?;
		if self.is_before(other_position)? {
			return -1;
		}
		if self.is_after(other_position)? {
			return 1;
		}
		return 0;
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if o == null || self.getClass() != o.getClass() {
			return false;
		}
	
		let other_position: Position = o as Position;
		return Objects::equals(self.line, other_position.line) && Objects::equals(self.column, other_position.column);
	}

	pub fn hash_code(&self) -> i32 {
		return Objects::hash(self.line, self.column);
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "(line " + self.line + ",col " + self.column + ")";
	}
}

impl /* Java */ java::lang::Comparable /**/ for Position {}