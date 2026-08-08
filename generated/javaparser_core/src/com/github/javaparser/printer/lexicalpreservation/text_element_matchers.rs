use crate::com::github::javaparser::ast::Node;

struct TextElementMatchers;

impl TextElementMatchers {
	fn by_token_type(&self, token_type: i32) -> com::github::javaparser::printer::lexicalpreservation::text_element_matcher::TextElementMatcher {
		return |text_element|text_element.is_token(token_type);
	}

	fn by_node(&self, node: &com::github::javaparser::ast::node::Node) -> com::github::javaparser::printer::lexicalpreservation::text_element_matcher::TextElementMatcher {
		return TextElementMatcher::new() {
			pub fn match(&self, text_element: &TextElement) -> bool {
				return text_element.is_node(node);
			}
	
			pub fn to_string(&self) -> String {
				return "match node " + node;
			}
	
		};
	}

	pub fn match(&self, text_element: &com::github::javaparser::printer::lexicalpreservation::text_element::TextElement) -> bool {
		return text_element.is_node(node);
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "match node " + node;
	}
}