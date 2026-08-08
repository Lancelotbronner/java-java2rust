use crate::com::github::javaparser::GeneratedJavaParserConstants;
use crate::com::github::javaparser::utils::LineSeparator;

pub struct TokenTypes;

impl TokenTypes {
	pub fn is_whitespace(&self, kind: i32) /* thrown(java.lang.AssertionError) */ -> bool {
		return com::github::javaparser::token_types::TokenTypes::get_category(kind)?.is_whitespace();
	}

	pub fn is_end_of_line_token(&self, kind: i32) /* thrown(java.lang.AssertionError) */ -> bool {
		return com::github::javaparser::token_types::TokenTypes::get_category(kind)?.is_end_of_line();
	}

	pub fn is_whitespace_or_comment(&self, kind: i32) /* thrown(java.lang.AssertionError) */ -> bool {
		return com::github::javaparser::token_types::TokenTypes::get_category(kind)?.is_whitespace_or_comment();
	}

	pub fn is_space_or_tab(&self, kind: i32) -> bool {
		return com::github::javaparser::token_types::TokenTypes::is_whitespace_but_not_end_of_line(kind);
	}

	pub fn is_whitespace_but_not_end_of_line(&self, kind: i32) /* thrown(java.lang.AssertionError) */ -> bool {
		return com::github::javaparser::token_types::TokenTypes::get_category(kind)?.is_whitespace_but_not_end_of_line();
	}

	pub fn is_comment(&self, kind: i32) /* thrown(java.lang.AssertionError) */ -> bool {
		return com::github::javaparser::token_types::TokenTypes::get_category(kind)?.is_comment();
	}

	pub fn eol_token_kind(&self, line_separator: &com::github::javaparser::utils::line_separator::LineSeparator) /* thrown(java.lang.AssertionError) */ -> i32 {
		if line_separator.equals_string(LineSeparator::LF) {
			return ;
		}
		if line_separator.equals_string(LineSeparator::CRLF) {
			return ;
		}
		if line_separator.equals_string(LineSeparator::CR) {
			return ;
		}
		return Err(AssertionError::new("Unknown EOL character sequence"));
	}

	pub fn eol_token_kind(&self) /* thrown(java.lang.AssertionError) */ -> i32 {
		return com::github::javaparser::token_types::TokenTypes::eol_token_kind(LineSeparator::SYSTEM)?;
	}

	pub fn space_token_kind(&self) -> i32 {
		return ;
	}

	pub fn get_category(&self, kind: i32) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::java_token::Category {
		match kind {
			 =>  {
			}
			 =>  {
			}
			 =>  {
				return JavaToken::com::github::javaparser::java_token::Category::EOL;
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
				return JavaToken::com::github::javaparser::java_token::Category::WHITESPACE_NO_EOL;
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
				return JavaToken::com::github::javaparser::java_token::Category::COMMENT;
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
				return JavaToken::com::github::javaparser::java_token::Category::KEYWORD;
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
				return JavaToken::com::github::javaparser::java_token::Category::LITERAL;
			}
			 =>  {
				return JavaToken::com::github::javaparser::java_token::Category::IDENTIFIER;
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
				return JavaToken::com::github::javaparser::java_token::Category::SEPARATOR;
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
				return JavaToken::com::github::javaparser::java_token::Category::OPERATOR;
			}
			// The following are tokens that are only used internally by the lexer
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			 =>  {
			}
			_ =>  {
				return Err(AssertionError::new("Unable to categorise token kind " + kind + " -- has it recently been added to the grammar but not classified within TokenTypes.java, perhaps?"));
			}
		}
	}
}