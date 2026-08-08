use crate::com::github::javaparser::JavaToken;
use crate::com::github::javaparser::JavaToken::Kind;
use crate::com::github::javaparser::Range;
use crate::com::github::javaparser::ast::Node;
use java::util::Optional;

pub struct TokenTextElement {
	token: com::github::javaparser::java_token::JavaToken,
}

impl TokenTextElement {
	fn new(token: &com::github::javaparser::java_token::JavaToken) -> com::github::javaparser::printer::lexicalpreservation::token_text_element::TokenTextElement {
		self.token = token;
	}

	fn new(token_kind: i32, text: &/* Java */ java::lang::String /**/) -> com::github::javaparser::printer::lexicalpreservation::token_text_element::TokenTextElement {
		this(JavaToken::new(token_kind, text));
	}

	fn new(token_kind: i32) -> com::github::javaparser::printer::lexicalpreservation::token_text_element::TokenTextElement {
		this(JavaToken::new(token_kind));
	}

	pub fn expand(&self) -> /* Java */ java::lang::String /**/ {
		return self.token.get_text();
	}

	pub fn get_text(&self) -> /* Java */ java::lang::String /**/ {
		return self.token.get_text();
	}

	pub fn get_token_kind(&self) -> i32 {
		return self.token.get_kind();
	}

	pub fn get_token(&self) -> com::github::javaparser::java_token::JavaToken {
		return self.token;
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if o == null || self.getClass() != o.getClass() {
			return false;
		}
	
		let that: TokenTextElement = o as TokenTextElement;
		return self.token.equals(that.token);
	}

	pub fn hash_code(&self) -> i32 {
		return self.token.hash_code();
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.token.to_string();
	}

	fn is_token(&self, token_kind: i32) -> bool {
		return self.token.get_kind() == token_kind;
	}

	fn is_node(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		return false;
	}

	pub fn is_white_space(&self) /* thrown(java.lang.AssertionError) */ -> bool {
		return self.token.get_category()?.is_whitespace();
	}

	pub fn is_space_or_tab(&self) /* thrown(java.lang.AssertionError) */ -> bool {
		return self.token.get_category()?.is_whitespace_but_not_end_of_line();
	}

	pub fn is_comment(&self) /* thrown(java.lang.AssertionError) */ -> bool {
		return self.token.get_category()?.is_comment();
	}

	pub fn is_separator(&self) /* thrown(java.lang.AssertionError) */ -> bool {
		return self.token.get_category()?.is_separator();
	}

	pub fn is_newline(&self) /* thrown(java.lang.AssertionError) */ -> bool {
		return self.token.get_category()?.is_end_of_line();
	}

	pub fn is_child_of_class(&self, node_class: &/* Java */ java::lang::Class /**/) -> bool {
		return false;
	}

	pub fn is_identifier(&self) /* thrown(java.lang.AssertionError) */ -> bool {
		return self.get_token().get_category()?.is_identifier();
	}

	pub fn is_keyword(&self) /* thrown(java.lang.AssertionError) */ -> bool {
		return self.get_token().get_category()?.is_keyword();
	}

	pub fn is_literal(&self) /* thrown(java.lang.AssertionError) */ -> bool {
		return self.get_token().get_category()?.is_literal();
	}

	pub fn is_primitive(&self) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		return Kind::value_of(&self.get_token_kind())?.is_primitive();
	}

	fn get_range(&self) -> /* Java */ java::util::Optional /**/ {
		return self.token.get_range();
	}

	pub fn accept(&self, visitor: &com::github::javaparser::printer::lexicalpreservation::lexical_preserving_visitor::LexicalPreservingVisitor) {
		visitor.visit(self);
	}
}

impl com::github::javaparser::printer::lexicalpreservation::text_element_matcher::TextElementMatcher for TokenTextElement {}

impl com::github::javaparser::printer::lexicalpreservation::printable_text_element::PrintableTextElement for TokenTextElement {}