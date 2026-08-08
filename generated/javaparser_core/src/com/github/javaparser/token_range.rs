use crate::com::github::javaparser::utils::Utils::assertNotNull;
use java::util::Iterator;
use java::util::Optional;

pub struct TokenRange {
	begin: com::github::javaparser::java_token::JavaToken,
	end: com::github::javaparser::java_token::JavaToken,
	has_next: bool = true,
	current: com::github::javaparser::java_token::JavaToken = begin,
}

impl TokenRange {
	pub static INVALID: com::github::javaparser::token_range::TokenRange = TokenRange::new(JavaToken::com::github::javaparser::java_token::JavaToken::INVALID, JavaToken::com::github::javaparser::java_token::JavaToken::INVALID);

	pub fn new(begin: &com::github::javaparser::java_token::JavaToken, end: &com::github::javaparser::java_token::JavaToken) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::token_range::TokenRange {
		self.begin = com::github::javaparser::utils::utils::Utils::assert_not_null(begin)?;
		self.end = com::github::javaparser::utils::utils::Utils::assert_not_null(end)?;
	}

	pub fn get_begin(&self) -> com::github::javaparser::java_token::JavaToken {
		return self.begin;
	}

	pub fn get_end(&self) -> com::github::javaparser::java_token::JavaToken {
		return self.end;
	}

	pub fn to_range(&self) -> /* Java */ java::util::Optional /**/ {
		if self.begin.has_range() && self.end.has_range() {
			return Optional::of(Range::new(self.begin.get_range().get().begin, self.end.get_range().get().end));
		}
		return Optional::empty();
	}

	pub fn with_begin(&self, begin: &com::github::javaparser::java_token::JavaToken) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::token_range::TokenRange {
		return TokenRange::new(&com::github::javaparser::utils::utils::Utils::assert_not_null(begin)?, self.end);
	}

	pub fn with_end(&self, end: &com::github::javaparser::java_token::JavaToken) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::token_range::TokenRange {
		return TokenRange::new(self.begin, &com::github::javaparser::utils::utils::Utils::assert_not_null(end)?);
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		let result: StringBuilder = StringBuilder::new();
		for t in self {
			result.append(&t.get_text());
		}
		return result.toString();
	}

	pub fn iterator(&self) -> /* Java */ java::util::Iterator /**/ {
		return Iterator<JavaToken>::new() {
			let has_next: bool = true,
			let current: JavaToken = self.begin,
			pub fn has_next(&self) -> bool {
				return self.has_next;
			}
	
			pub fn next(&self) -> JavaToken {
				let retval: JavaToken = self.current;
				if self.current == null {
					return Err(IllegalStateException::new("Attempting to move past end of range."));
				}
				if self.current == self.end {
					self.has_next = false;
				}
				self.current = self.current.get_next_token().orElse(null);
				if self.current == null && self.has_next {
					return Err(IllegalStateException::new("End token is not linked to begin token."));
				}
				return retval;
			}
	
		};
	}

	pub fn has_next(&self) -> bool {
		return self.has_next;
	}

	pub fn next(&mut self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::java_token::JavaToken {
		let retval: JavaToken = self.current;
		if self.current == null {
			return Err(IllegalStateException::new("Attempting to move past end of range."));
		}
		if self.current == self.end {
			self.has_next = false;
		}
		self.current = self.current.get_next_token().orElse(null);
		if self.current == null && self.has_next {
			return Err(IllegalStateException::new("End token is not linked to begin token."));
		}
		return retval;
	}
}

impl /* Java */ java::lang::Iterable /**/ for TokenRange {}