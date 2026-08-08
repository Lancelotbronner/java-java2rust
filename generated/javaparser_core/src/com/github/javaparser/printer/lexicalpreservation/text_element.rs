use crate::com::github::javaparser::GeneratedJavaParserConstants;
use crate::com::github::javaparser::Range;
use crate::com::github::javaparser::ast::Node;
use java::util::Optional;

pub struct TextElement;

impl TextElement {
	fn expand(&self) -> /* Java */ java::lang::String /**/ ;

	fn is_token(&self, token_kind: i32) -> bool ;

	fn is_comment_token(&self) -> bool {
		return self.is_token(GeneratedJavaParserConstants.JAVADOC_COMMENT) || self.is_token(GeneratedJavaParserConstants.SINGLE_LINE_COMMENT) || self.is_token(GeneratedJavaParserConstants.MULTI_LINE_COMMENT);
	}

	pub fn match(&self, text_element: &com::github::javaparser::printer::lexicalpreservation::text_element::TextElement) -> bool {
		return self.equals(text_element);
	}

	fn is_node(&self, node: &com::github::javaparser::ast::node::Node) -> bool ;

	pub fn is_literal(&self) -> bool ;

	pub fn is_white_space(&self) -> bool ;

	pub fn is_space_or_tab(&self) -> bool ;

	pub fn is_newline(&self) -> bool ;

	pub fn is_comment(&self) -> bool ;

	pub fn is_separator(&self) -> bool ;

	pub fn is_identifier(&self) -> bool ;

	pub fn is_keyword(&self) -> bool ;

	pub fn is_primitive(&self) -> bool ;

	pub fn is_white_space_or_comment(&self) -> bool {
		return self.is_white_space() || self.is_comment();
	}

	pub fn is_child_of_class(&self, node_class: &/* Java */ java::lang::Class /**/) -> bool ;

	pub fn is_child(&self) -> bool {
		return self.is_child_of_class(Node.class);
	}

	fn get_range(&self) -> /* Java */ java::util::Optional /**/ ;

	fn match_by_range(&self) -> com::github::javaparser::printer::lexicalpreservation::text_element_matcher::TextElementMatcher {
		return |(// harsh on that:
		text_element: &TextElement)|self.get_range().flatMap(|r1|text_element.get_range().map(r1::equals)).orElse(true);
	}
}

impl com::github::javaparser::printer::lexicalpreservation::text_element_matcher::TextElementMatcher for TextElement {}

impl com::github::javaparser::printer::lexicalpreservation::printable_text_element::PrintableTextElement for TextElement {}