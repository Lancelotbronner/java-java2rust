use crate::com::github::javaparser::utils::CodeGenerationUtils::f;
use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::utils::LineSeparator;
use java::util::List;
use java::util::Optional;

pub struct JavaToken {
	range: com::github::javaparser::range::Range,
	kind: i32,
	text: /* Java */ java::lang::String /**/,
	previous_token: com::github::javaparser::java_token::JavaToken = null,
	next_token: com::github::javaparser::java_token::JavaToken = null,
}

impl JavaToken {
	pub static INVALID: com::github::javaparser::java_token::JavaToken = JavaToken::new();

	fn new() -> com::github::javaparser::java_token::JavaToken {
		this(null, 0, "INVALID", null, null);
	}

	pub fn new(kind: i32, text: &/* Java */ java::lang::String /**/) -> com::github::javaparser::java_token::JavaToken {
		this(null, kind, text, null, null);
	}

	fn new(token: &com::github::javaparser::token::Token, tokens: &/* Java */ java::util::List /**/) -> com::github::javaparser::java_token::JavaToken {
		// You could be puzzled by the following lines
		//
		// The reason why these lines are necessary is the fact that Java is ambiguous. There are cases where the
		// sequence of characters ">>>" and ">>" should be recognized as the single tokens ">>>" and ">>". In other
		// cases however we want to split those characters in single GT tokens (">").
		//
		// For example, in expressions ">>" and ">>>" are valid, while when defining types we could have this:
		//
		// List<List<Set<String>>>>
		//
		// You can see that the sequence ">>>>" should be interpreted as four consecutive ">" tokens closing a type
		// parameter list.
		//
		// The JavaCC handle this case by first recognizing always the longest token, and then depending on the context
		// putting back the unused chars in the stream. However in those cases the token provided is invalid: it has an
		// image corresponding to the text originally recognized, without considering that after some characters could
		// have been put back into the stream.
		//
		// So in the case of:
		//
		// List<List<Set<String>>>>
		// ___   -> recognized as ">>>", then ">>" put back in the stream but Token(type=GT, image=">>>") passed to this
		// class
		// ___  -> recognized as ">>>", then ">>" put back in the stream but Token(type=GT, image=">>>") passed to this
		// class
		// __  -> recognized as ">>", then ">" put back in the stream but Token(type=GT, image=">>") passed to this
		// class
		// _  -> Token(type=GT, image=">") good!
		//
		// So given the image could be wrong but the type is correct, we look at the type of the token and we fix
		// the image. Everybody is happy and we can keep this horrible thing as our little secret.
		let range: Range = Range::range(token.beginLine, token.beginColumn, token.endLine, token.endColumn);
		let text: String = token.image;
		if token.kind == GeneratedJavaParserConstants.GT {
			range = Range::range(token.beginLine, token.beginColumn, token.endLine, token.beginColumn);
			text = ">";
		} else if token.kind == GeneratedJavaParserConstants.RSIGNEDSHIFT {
			range = Range::range(token.beginLine, token.beginColumn, token.endLine, token.beginColumn + 1);
			text = ">>";
		}
		self.range = range;
		self.kind = token.kind;
		self.text = text;
		if !tokens.isEmpty() {
			/* final */ let previous_token: JavaToken = tokens.get(tokens.size() - 1);
			self.previous_token = previous_token;
			previous_token.next_token = self;
		} else {
			self.previous_token = null;
		}
	}

	pub fn new(kind: i32) -> com::github::javaparser::java_token::JavaToken {
		let content: String = GeneratedJavaParserConstants.tokenImage[kind];
		if content.startsWith("\"") {
			content = content.substring(1, content.length() - 1);
		}
		if TokenTypes::is_end_of_line_token(kind) {
			content = LineSeparator::SYSTEM.as_raw_string();
		} else if TokenTypes::is_whitespace(kind) {
			content = " ";
		}
		self.kind = kind;
		self.text = content;
	}

	pub fn new(range: &com::github::javaparser::range::Range, kind: i32, text: &/* Java */ java::lang::String /**/, previous_token: &com::github::javaparser::java_token::JavaToken, next_token: &com::github::javaparser::java_token::JavaToken) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::java_token::JavaToken {
		com::github::javaparser::utils::utils::Utils::assert_not_null(text)?;
		self.range = range;
		self.kind = kind;
		self.text = text;
		self.previous_token = previous_token;
		self.next_token = next_token;
	}

	pub fn get_range(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.range);
	}

	pub fn has_range(&self) -> bool {
		return self.get_range().isPresent();
	}

	pub fn get_kind(&self) -> i32 {
		return self.kind;
	}

	fn set_kind(&mut self, kind: i32) {
		self.kind = kind;
	}

	pub fn get_text(&self) -> /* Java */ java::lang::String /**/ {
		return self.text;
	}

	pub fn get_next_token(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.next_token);
	}

	pub fn get_previous_token(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.previous_token);
	}

	pub fn set_range(&mut self, range: &com::github::javaparser::range::Range) {
		self.range = range;
	}

	pub fn set_text(&mut self, text: &/* Java */ java::lang::String /**/) {
		self.text = text;
	}

	pub fn as_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.text;
	}

	pub fn to_token_range(&self) -> com::github::javaparser::token_range::TokenRange {
		return TokenRange::new(&self.find_first_token(), &self.find_last_token());
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		let text: String = self.get_text().replace("\n", "\\n").replace("\r", "\\r").replace("\r\n", "\\r\\n").replace("\t", "\\t");
		return com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("\"%s\"   <%s>   %s", text, &self.get_kind(), &self.get_range().map(Range::toString).orElse("(?)-(?)"));
	}

	pub fn valid(&self) -> bool {
		return !self.invalid();
	}

	pub fn invalid(&self) -> bool {
		return self == self.INVALID;
	}

	pub fn get_category(&self) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::java_token::Category {
		return TokenTypes::get_category(self.kind)?;
	}

	pub fn insert(&self, new_token: &com::github::javaparser::java_token::JavaToken) /* thrown(java.lang.AssertionError) */ {
		com::github::javaparser::utils::utils::Utils::assert_not_null(new_token)?;
		self.get_previous_token().ifPresent(|p|{
			p.next_token = new_token;
			new_token.previous_token = p;
		});
		self.previous_token = new_token;
		new_token.next_token = self;
	}

	pub fn insert_after(&self, mut new_token: &com::github::javaparser::java_token::JavaToken) /* thrown(java.lang.AssertionError) */ {
		com::github::javaparser::utils::utils::Utils::assert_not_null(new_token)?;
		self.get_next_token().ifPresent(|n|{
			n.previous_token = new_token;
			new_token.next_token = n;
		});
		self.next_token = new_token;
		new_token.previous_token = self;
	}

	pub fn delete_token(&self) {
		/* final */ let next_token: Optional<JavaToken> = self.get_next_token();
		/* final */ let previous_token: Optional<JavaToken> = self.get_previous_token();
		previous_token.ifPresent(|p|p.next_token = next_token.orElse(null));
		next_token.ifPresent(|n|n.previous_token = previous_token.orElse(null));
	}

	pub fn replace_token(&self, mut new_token: &com::github::javaparser::java_token::JavaToken) /* thrown(java.lang.AssertionError) */ {
		com::github::javaparser::utils::utils::Utils::assert_not_null(new_token)?;
		self.get_previous_token().ifPresent(|p|{
			p.next_token = new_token;
			new_token.previous_token = p;
		});
		self.get_next_token().ifPresent(|n|{
			n.previous_token = new_token;
			new_token.next_token = n;
		});
	}

	pub fn find_last_token(&self) -> com::github::javaparser::java_token::JavaToken {
		let current: JavaToken = self;
		while current.get_next_token().isPresent() {
			current = current.get_next_token().get();
		}
		return current;
	}

	pub fn find_first_token(&self) -> com::github::javaparser::java_token::JavaToken {
		let current: JavaToken = self;
		while current.get_previous_token().isPresent() {
			current = current.get_previous_token().get();
		}
		return current;
	}

	pub fn hash_code(&self) -> i32 {
		let result: i32 = self.kind;
		result = 31 * result + self.text.hashCode();
		return result;
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if o == null || self.getClass() != o.getClass() {
			return false;
		}
	
		let java_token: JavaToken = o as JavaToken;
		if self.kind != java_token.kind {
			return false;
		}
	
		if !self.text.equals(java_token.text) {
			return false;
		}
	
		return true;
	}
}

pub enum Category;

pub enum Kind {
	kind: i32,
}