use crate::com::github::javaparser::utils::Pair;
use java::util::LinkedList;
use java::util::List;

pub struct JavadocDescription {
	elements: /* Java */ java::util::List /**/,
}

impl JavadocDescription {
	pub fn parse_text(&self, text: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::javadoc::description::javadoc_description::JavadocDescription {
		let instance: JavadocDescription = JavadocDescription::new();
		let index: i32 = 0;
		let next_inline_tag_pos: Pair<Integer, Integer>;
		while (next_inline_tag_pos = com::github::javaparser::javadoc::description::javadoc_description::JavadocDescription::index_of_next_inline_tag(text, index)) != null {
			if next_inline_tag_pos.a != index {
				instance.add_element(JavadocSnippet::new(&text.substring(index, next_inline_tag_pos.a)));
			}
			instance.add_element(&JavadocInlineTag::from_text(&text.substring(next_inline_tag_pos.a, next_inline_tag_pos.b + 1))?);
			index = next_inline_tag_pos.b + 1;
		}
		if index < text.length() {
			instance.add_element(JavadocSnippet::new(&text.substring(index)));
		}
		return instance;
	}

	fn index_of_next_inline_tag(&self, text: &/* Java */ java::lang::String /**/, start: i32) -> com::github::javaparser::utils::pair::Pair {
		let index: i32 = text.indexOf("{@", start);
		if index == -1 {
			return null;
		}
		// we are interested only in complete inline tags
		let close_index: i32 = text.indexOf("}", index);
		if close_index == -1 {
			return null;
		}
		return Pair<>::new(index, close_index);
	}

	pub fn new() -> com::github::javaparser::javadoc::description::javadoc_description::JavadocDescription {
		self.elements = LinkedList<>::new();
	}

	pub fn new(elements: &/* Java */ java::util::List /**/) -> com::github::javaparser::javadoc::description::javadoc_description::JavadocDescription {
		this();
		self.elements.addAll(elements);
	}

	pub fn add_element(&self, element: &com::github::javaparser::javadoc::description::javadoc_description_element::JavadocDescriptionElement) -> bool {
		return self.elements.add(element);
	}

	pub fn get_elements(&self) -> /* Java */ java::util::List /**/ {
		return self.elements;
	}

	pub fn to_text(&self) -> /* Java */ java::lang::String /**/ {
		let sb: StringBuilder = StringBuilder::new();
		self.elements.forEach(|e|sb.append(&e.to_text()));
		return sb.toString();
	}

	pub fn is_empty(&self) -> bool {
		return self.to_text().isEmpty();
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if o == null || self.getClass() != o.getClass() {
			return false;
		}
	
		let that: JavadocDescription = o as JavadocDescription;
		return self.elements.equals(that.elements);
	}

	pub fn hash_code(&self) -> i32 {
		return self.elements.hashCode();
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "JavadocDescription{" + "elements=" + self.elements + '}';
	}
}