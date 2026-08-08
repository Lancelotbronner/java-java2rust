use crate::com::github::javaparser::utils::Utils::nextWord;
use crate::com::github::javaparser::utils::Utils::screamingToCamelCase;
use crate::com::github::javaparser::javadoc::description::JavadocDescription;
use java::util::Optional;

pub struct JavadocBlockTag {
	type: com::github::javaparser::javadoc::javadoc_block_tag::Type,
	content: com::github::javaparser::javadoc::description::javadoc_description::JavadocDescription,
	name: /* Java */ java::util::Optional /**/ = Optional::empty(),
	tag_name: /* Java */ java::lang::String /**/,
}

impl JavadocBlockTag {
	pub fn new(type: &com::github::javaparser::javadoc::javadoc_block_tag::Type, mut content: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::javadoc::javadoc_block_tag::JavadocBlockTag {
		self.type = type;
		self.tagName = type.keyword;
		if type.has_name() {
			self.name = Optional::of(&com::github::javaparser::utils::utils::Utils::next_word(content));
			content = content.substring(&self.name.get().length()).trim();
		}
		self.content = JavadocDescription::parse_text(content)?;
	}

	pub fn new(tag_name: &/* Java */ java::lang::String /**/, content: &/* Java */ java::lang::String /**/) -> com::github::javaparser::javadoc::javadoc_block_tag::JavadocBlockTag {
		this(&Type::from_name(tag_name), content);
		self.tagName = tag_name;
	}

	pub fn create_param_block_tag(&self, param_name: &/* Java */ java::lang::String /**/, content: &/* Java */ java::lang::String /**/) -> com::github::javaparser::javadoc::javadoc_block_tag::JavadocBlockTag {
		return JavadocBlockTag::new(Type::PARAM, param_name + " " + content);
	}

	pub fn get_type(&self) -> com::github::javaparser::javadoc::javadoc_block_tag::Type {
		return self.type;
	}

	pub fn get_content(&self) -> com::github::javaparser::javadoc::description::javadoc_description::JavadocDescription {
		return self.content;
	}

	pub fn get_name(&self) -> /* Java */ java::util::Optional /**/ {
		return self.name;
	}

	pub fn get_tag_name(&self) -> /* Java */ java::lang::String /**/ {
		return self.tag_name;
	}

	pub fn to_text(&self) -> /* Java */ java::lang::String /**/ {
		let sb: StringBuilder = StringBuilder::new();
		sb.append("@");
		sb.append(self.tag_name);
		self.name.ifPresent(|s|sb.append(" ").append(s));
		if !self.content.is_empty() {
			sb.append(" ");
			sb.append(&self.content.to_text());
		}
		return sb.toString();
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if o == null || self.getClass() != o.getClass() {
			return false;
		}
	
		let that: JavadocBlockTag = o as JavadocBlockTag;
		if self.type != that.type {
			return false;
		}
	
		if !self.content.equals(that.content) {
			return false;
		}
	
		return self.name.equals(that.name);
	}

	pub fn hash_code(&self) -> i32 {
		let result: i32 = self.type.hashCode();
		result = 31 * result + self.content.hash_code();
		result = 31 * result + self.name.hashCode();
		return result;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "JavadocBlockTag{" + "type=" + self.type + ", content='" + self.content + '\'' + ", name=" + self.name + '}';
	}
}

pub enum Type {
	keyword: /* Java */ java::lang::String /**/,
}