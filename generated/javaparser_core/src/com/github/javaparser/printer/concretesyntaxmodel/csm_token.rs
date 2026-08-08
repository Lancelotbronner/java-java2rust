use crate::com::github::javaparser::TokenTypes::isEndOfLineToken;
use crate::com::github::javaparser::TokenTypes::isWhitespaceButNotEndOfLine;
use crate::com::github::javaparser::GeneratedJavaParserConstants;
use crate::com::github::javaparser::TokenTypes;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::printer::SourcePrinter;
use crate::com::github::javaparser::printer::lexicalpreservation::TextElement;
use crate::com::github::javaparser::printer::lexicalpreservation::TokenTextElement;
use crate::com::github::javaparser::utils::LineSeparator;

pub struct CsmToken {
	token_type: i32,
	content: /* Java */ java::lang::String /**/,
}

impl CsmToken {
	pub fn get_token_type(&self) -> i32 {
		return self.token_type;
	}

	pub fn get_content(&self) -> /* Java */ java::lang::String /**/ {
		return self.content;
	}

	pub fn new(token_type: i32) -> com::github::javaparser::printer::concretesyntaxmodel::csm_token::CsmToken {
		self.tokenType = token_type;
		self.content = GeneratedJavaParserConstants.tokenImage[token_type];
		if self.content.startsWith("\"") {
			self.content = self.content.substring(1, self.content.length() - 1);
		}
		// and "placeholder" values ({@code <SPACE>}) with their textual counterparts
		if com::github::javaparser::token_types::TokenTypes::is_end_of_line_token(token_type) {
			// Use the unescaped version
			self.content = LineSeparator::lookup_escaped(self.content).get().as_raw_string();
		} else if com::github::javaparser::token_types::TokenTypes::is_whitespace_but_not_end_of_line(token_type) {
			self.content = " ";
		}
	}

	pub fn new(token_type: i32, content: &/* Java */ java::lang::String /**/) -> com::github::javaparser::printer::concretesyntaxmodel::csm_token::CsmToken {
		self.tokenType = token_type;
		self.content = content;
	}

	pub fn pretty_print(&self, node: &com::github::javaparser::ast::node::Node, printer: &com::github::javaparser::printer::source_printer::SourcePrinter) {
		if com::github::javaparser::token_types::TokenTypes::is_end_of_line_token(self.token_type) {
			printer.println();
		} else {
			printer.print(&self.get_content());
		}
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return String::format("%s(property:%s)", &self.getClass().getSimpleName(), self.content);
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if o == null || self.getClass() != o.getClass() {
			return false;
		}
	
		let csm_token: CsmToken = o as CsmToken;
		if self.token_type != csm_token.tokenType {
			return false;
		}
	
		if  if self.content != null { !self.content.equals(csm_token.content) } else { csm_token.content != null } {
			return false;
		}
	
		return true;
	}

	pub fn hash_code(&self) -> i32 {
		let result: i32 = self.token_type;
		result = 31 * result + ( if self.content != null { self.content.hashCode() } else { 0 });
		return result;
	}

	pub fn is_white_space(&self) -> bool {
		return TokenTypes::is_whitespace(self.token_type);
	}

	pub fn is_white_space_not_eol(&self) -> bool {
		return self.is_white_space() && !self.is_new_line();
	}

	pub fn is_new_line(&self) -> bool {
		return TokenTypes::is_end_of_line_token(self.token_type);
	}

	pub fn is_corresponding_element(&self, text_element: &com::github::javaparser::printer::lexicalpreservation::text_element::TextElement) -> bool {
		return (text_element instanceof TokenTextElement) && (text_element as TokenTextElement).get_token_kind() == self.get_token_type() && (text_element as TokenTextElement).get_text().equals(&self.get_content());
	}
}

impl com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement for CsmToken {}