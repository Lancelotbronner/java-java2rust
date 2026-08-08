use crate::com::github::javaparser::ast::Node;
use java::util::Collection;
use java::util::LinkedList;
use java::util::List;

struct NodeText {
	elements: /* Java */ java::util::List /**/,
}

impl NodeText {
	pub static NOT_FOUND: i32 = -1;

	fn new(elements: &/* Java */ java::util::List /**/) -> com::github::javaparser::printer::lexicalpreservation::node_text::NodeText {
		self.elements = elements;
	}

	fn new() -> com::github::javaparser::printer::lexicalpreservation::node_text::NodeText {
		this(LinkedList<>::new());
	}

	fn add_element(&self, node_text_element: &com::github::javaparser::printer::lexicalpreservation::text_element::TextElement) {
		self.elements.add(node_text_element);
	}

	fn add_element(&self, index: i32, node_text_element: &com::github::javaparser::printer::lexicalpreservation::text_element::TextElement) {
		self.elements.add(index, node_text_element);
	}

	fn add_child(&self, child: &com::github::javaparser::ast::node::Node) {
		self.add_element(ChildTextElement::new(child));
	}

	fn add_child(&self, index: i32, child: &com::github::javaparser::ast::node::Node) {
		self.add_element(index, ChildTextElement::new(child));
	}

	fn add_token(&self, token_kind: i32, text: &/* Java */ java::lang::String /**/) {
		self.elements.add(TokenTextElement::new(token_kind, text));
	}

	fn add_token(&self, index: i32, token_kind: i32, text: &/* Java */ java::lang::String /**/) {
		self.elements.add(index, TokenTextElement::new(token_kind, text));
	}

	fn find_element(&self, matcher: &com::github::javaparser::printer::lexicalpreservation::text_element_matcher::TextElementMatcher) /* thrown(java.lang.IllegalArgumentException) */ -> i32 {
		return self.find_element(matcher, 0)?;
	}

	fn find_element(&self, matcher: &com::github::javaparser::printer::lexicalpreservation::text_element_matcher::TextElementMatcher, from: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i32 {
		let res: i32 = self.try_to_find_element(matcher, from);
		if res == self.NOT_FOUND {
			return Err(IllegalArgumentException::new(&String::format("I could not find child '%s' from position %d. Elements: %s", matcher, from, self.elements)));
		}
		return res;
	}

	fn try_to_find_element(&self, matcher: &com::github::javaparser::printer::lexicalpreservation::text_element_matcher::TextElementMatcher, from: i32) -> i32 {
		 {
			let i: i32 = from;
			while i < self.elements.size() {
				{
					let element: TextElement = self.elements.get(i);
					if matcher.match(element) {
						return i;
					}
				}
				i += 1;
			 }
		 }
	
		return self.NOT_FOUND;
	}

	fn find_child(&self, child: &com::github::javaparser::ast::node::Node) -> i32 {
		return self.find_child(child, 0);
	}

	fn find_child(&self, child: &com::github::javaparser::ast::node::Node, from: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i32 {
		return self.find_element(&TextElementMatchers::by_node(child), from)?;
	}

	fn try_to_find_child(&self, child: &com::github::javaparser::ast::node::Node) -> i32 {
		return self.try_to_find_child(child, 0);
	}

	fn try_to_find_child(&self, child: &com::github::javaparser::ast::node::Node, from: i32) -> i32 {
		return self.try_to_find_element(&TextElementMatchers::by_node(child), from);
	}

	pub fn remove(&self, matcher: &com::github::javaparser::printer::lexicalpreservation::text_element_matcher::TextElementMatcher, potentially_following_whitespace: bool) /* thrown(java.lang.UnsupportedOperationException | java.lang.IllegalArgumentException) */ {
		let i: i32 = 0;
		for e in self.elements {
			if matcher.match(e) {
				self.elements.remove(e);
				if potentially_following_whitespace {
					if i < self.elements.size() {
						if self.elements.get(i).is_white_space() {
							self.elements.remove(i);
						}
					} else {
						return Err(UnsupportedOperationException::new("There is no element to remove!"));
					}
				}
				return;
			}
		}
		return Err(IllegalArgumentException::new());
	}

	fn remove_element(&self, index: i32) {
		self.elements.remove(index);
	}

	fn replace(&self, position: &com::github::javaparser::printer::lexicalpreservation::text_element_matcher::TextElementMatcher, new_element: &com::github::javaparser::printer::lexicalpreservation::text_element::TextElement) {
		let index: i32 = self.find_element(position, 0)?;
		self.elements.remove(index);
		self.elements.add(index, new_element);
	}

	fn replace(&self, position: &com::github::javaparser::printer::lexicalpreservation::text_element_matcher::TextElementMatcher, new_elements: &/* Java */ java::util::Collection /**/) {
		let index: i32 = self.find_element(position, 0)?;
		self.elements.remove(index);
		self.elements.addAll(index, new_elements);
	}

	fn expand(&self) -> /* Java */ java::lang::String /**/ {
		let sb: StringBuffer = StringBuffer::new();
		self.elements.forEach(|e|sb.append(&e.expand()));
		return sb.toString();
	}

	fn number_of_elements(&self) -> i32 {
		return self.elements.size();
	}

	fn get_text_element(&self, index: i32) -> com::github::javaparser::printer::lexicalpreservation::text_element::TextElement {
		return self.elements.get(index);
	}

	fn get_elements(&self) -> /* Java */ java::util::List /**/ {
		return self.elements;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "NodeText{" + self.elements + '}';
	}
}