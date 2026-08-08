use crate::com::github::javaparser::utils::Utils::nextWord;
use crate::com::github::javaparser::utils::Utils::screamingToCamelCase;

pub struct JavadocInlineTag {
	tag_name: /* Java */ java::lang::String /**/,
	type: com::github::javaparser::javadoc::description::javadoc_inline_tag::Type,
	content: /* Java */ java::lang::String /**/,
}

impl JavadocInlineTag {
	pub fn from_text(&self, mut text: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::javadoc::description::javadoc_description_element::JavadocDescriptionElement {
		if !text.startsWith("{@") {
			return Err(IllegalArgumentException::new(&String::format("Expected to start with '{@'. Text '%s'", text)));
		}
		if !text.endsWith("}") {
			return Err(IllegalArgumentException::new(&String::format("Expected to end with '}'. Text '%s'", text)));
		}
		text = text.substring(2, text.length() - 1);
		let tag_name: String = com::github::javaparser::utils::utils::Utils::next_word(text);
		let type: Type = Type::from_name(tag_name);
		let content: String = text.substring(&tag_name.length());
		return JavadocInlineTag::new(tag_name, type, content);
	}

	pub fn new(tag_name: &/* Java */ java::lang::String /**/, type: &com::github::javaparser::javadoc::description::javadoc_inline_tag::Type, content: &/* Java */ java::lang::String /**/) -> com::github::javaparser::javadoc::description::javadoc_inline_tag::JavadocInlineTag {
		self.tagName = tag_name;
		self.type = type;
		self.content = content;
	}

	pub fn get_type(&self) -> com::github::javaparser::javadoc::description::javadoc_inline_tag::Type {
		return self.type;
	}

	pub fn get_content(&self) -> /* Java */ java::lang::String /**/ {
		return self.content;
	}

	pub fn get_name(&self) -> /* Java */ java::lang::String /**/ {
		return self.tag_name;
	}

	pub fn to_text(&self) -> /* Java */ java::lang::String /**/ {
		return "{@" + self.tag_name + self.content + "}";
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if o == null || self.getClass() != o.getClass() {
			return false;
		}
	
		let that: JavadocInlineTag = o as JavadocInlineTag;
		if  if self.tag_name != null { !self.tag_name.equals(that.tagName) } else { that.tagName != null } {
			return false;
		}
	
		if self.type != that.type {
			return false;
		}
	
		return  if self.content != null { self.content.equals(that.content) } else { that.content == null };
	}

	pub fn hash_code(&self) -> i32 {
		let result: i32 =  if self.tag_name != null { self.tag_name.hashCode() } else { 0 };
		result = 31 * result + ( if self.type != null { self.type.hashCode() } else { 0 });
		result = 31 * result + ( if self.content != null { self.content.hashCode() } else { 0 });
		return result;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "JavadocInlineTag{" + "tagName='" + self.tag_name + '\'' + ", type=" + self.type + ", content='" + self.content + '\'' + '}';
	}
}

impl com::github::javaparser::javadoc::description::javadoc_description_element::JavadocDescriptionElement for JavadocInlineTag {}

pub enum Type {
	keyword: /* Java */ java::lang::String /**/,
}