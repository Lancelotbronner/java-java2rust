use crate::com::github::javaparser::ast::comments;
use java::util::ArrayDeque;
use crate::com::github::javaparser::GeneratedJavaParserConstants;

struct GeneratedJavaParserTokenManagerBase;

impl GeneratedJavaParserTokenManagerBase {
	fn token_range(&self, token: &com::github::javaparser::token::Token) -> com::github::javaparser::token_range::TokenRange {
		let java_token: JavaToken = token.javaToken;
		return TokenRange::new(java_token, java_token);
	}

	fn is_markdown_comment_line_candidate(&self, token: &com::github::javaparser::token::Token) -> bool {
		return token.kind ==  && token.image.startsWith("///");
	}

	fn create_markdown_comment_from_token_list(&self, tokens: &/* Java */ java::util::ArrayDeque /**/) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::comments::markdown_comment::MarkdownComment {
		if tokens.isEmpty() {
			return Err(IllegalArgumentException::new("Cannot create markdown comment from empty token list"));
		}
		// should be filtered out.
		while !tokens.isEmpty() && TokenTypes::is_whitespace(tokens.peekLast().kind)? {
			let last_token: Token = tokens.removeLast();
			if TokenTypes::is_comment(last_token.kind)? {
				tokens.addLast(last_token);
				break;
			} else {
				if tokens.isEmpty() {
					return Err(IllegalArgumentException::new("createMarkdownCommentFromTokenList may not be called with a token list consisting only of whitespace tokens"));
				}
				if TokenTypes::is_end_of_line_token(last_token.kind)? {
					if TokenTypes::is_comment(tokens.peekLast().kind)? {
						break;
					}
				}
			}
		}
		let range: TokenRange = TokenRange::new(tokens.peekFirst().javaToken, tokens.peekLast().javaToken);
		let content_builder: StringBuilder = StringBuilder::new();
		for token in tokens {
			content_builder.append(token.image);
		}
		return MarkdownComment::new(range, &content_builder.toString());
	}

	fn create_comment_from_token(&self, token: &com::github::javaparser::token::Token) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::comments::comment::Comment {
		let comment_text: String = token.image;
		if token.kind ==  {
			return TraditionalJavadocComment::new(&com::github::javaparser::generated_java_parser_token_manager_base::GeneratedJavaParserTokenManagerBase::token_range(token), &comment_text.substring(3, comment_text.length() - 2));
		} else if token.kind ==  {
			return BlockComment::new(&com::github::javaparser::generated_java_parser_token_manager_base::GeneratedJavaParserTokenManagerBase::token_range(token), &comment_text.substring(2, comment_text.length() - 2));
		} else if token.kind ==  {
			return LineComment::new(&com::github::javaparser::generated_java_parser_token_manager_base::GeneratedJavaParserTokenManagerBase::token_range(token), &comment_text.substring(2));
		}
		return Err(AssertionError::new("Unexpectedly got passed a non-comment token."));
	}
}