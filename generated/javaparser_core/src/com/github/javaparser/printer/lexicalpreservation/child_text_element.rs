use crate::com::github::javaparser::Range;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::comments::Comment;
use java::util::Optional;

pub struct ChildTextElement {
	child: com::github::javaparser::ast::node::Node,
}

impl ChildTextElement {
	fn new(child: &com::github::javaparser::ast::node::Node) -> com::github::javaparser::printer::lexicalpreservation::child_text_element::ChildTextElement {
		self.child = child;
	}

	pub fn expand(&self) -> /* Java */ java::lang::String /**/ {
		return LexicalPreservingPrinter::print(self.child);
	}

	pub fn get_child(&self) -> com::github::javaparser::ast::node::Node {
		return self.child;
	}

	pub fn is_token(&self, token_kind: i32) -> bool {
		return false;
	}

	pub fn is_node(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		return node == self.child;
	}

	fn get_node_text_for_wrapped_node(&self) -> com::github::javaparser::printer::lexicalpreservation::node_text::NodeText {
		return LexicalPreservingPrinter::get_or_create_node_text(self.child);
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if o == null || self.getClass() != o.getClass() {
			return false;
		}
	
		let that: ChildTextElement = o as ChildTextElement;
		return self.child.equals(that.child);
	}

	pub fn hash_code(&self) -> i32 {
		return self.child.hash_code();
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "ChildTextElement{" + self.child + '}';
	}

	pub fn is_white_space(&self) -> bool {
		return false;
	}

	pub fn is_space_or_tab(&self) -> bool {
		return false;
	}

	pub fn is_newline(&self) -> bool {
		return false;
	}

	pub fn is_comment(&self) -> bool {
		return self.child instanceof Comment;
	}

	pub fn is_separator(&self) -> bool {
		return false;
	}

	pub fn is_identifier(&self) -> bool {
		return false;
	}

	pub fn is_keyword(&self) -> bool {
		return false;
	}

	pub fn is_primitive(&self) -> bool {
		return false;
	}

	pub fn is_literal(&self) -> bool {
		return false;
	}

	pub fn is_child_of_class(&self, node_class: &/* Java */ java::lang::Class /**/) -> bool {
		return node_class.isInstance(self.child);
	}

	fn get_range(&self) -> /* Java */ java::util::Optional /**/ {
		return self.child.get_range();
	}

	pub fn accept(&self, visitor: &com::github::javaparser::printer::lexicalpreservation::lexical_preserving_visitor::LexicalPreservingVisitor) {
		let node_text: NodeText = self.get_node_text_for_wrapped_node();
		node_text.get_elements().forEach(|element|element.accept(visitor));
	}
}

impl com::github::javaparser::printer::lexicalpreservation::text_element_matcher::TextElementMatcher for ChildTextElement {}

impl com::github::javaparser::printer::lexicalpreservation::printable_text_element::PrintableTextElement for ChildTextElement {}