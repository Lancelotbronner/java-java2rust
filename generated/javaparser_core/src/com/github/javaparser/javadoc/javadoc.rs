use crate::com::github::javaparser::ast::comments::JavadocComment;
use crate::com::github::javaparser::ast::comments::MarkdownComment;
use crate::com::github::javaparser::ast::comments::TraditionalJavadocComment;
use crate::com::github::javaparser::javadoc::description::JavadocDescription;
use crate::com::github::javaparser::utils::LineSeparator;
use java::util::LinkedList;
use java::util::List;

pub struct Javadoc {
	description: com::github::javaparser::javadoc::description::javadoc_description::JavadocDescription,
	block_tags: /* Java */ java::util::List /**/,
	is_markdown_comment: bool,
}

impl Javadoc {
	pub fn new(description: &com::github::javaparser::javadoc::description::javadoc_description::JavadocDescription) -> com::github::javaparser::javadoc::javadoc::Javadoc {
		self.description = description;
		self.blockTags = LinkedList<>::new();
	}

	pub fn new(description: &com::github::javaparser::javadoc::description::javadoc_description::JavadocDescription, is_markdown_comment: bool) -> com::github::javaparser::javadoc::javadoc::Javadoc {
		this(description);
		self.isMarkdownComment = is_markdown_comment;
	}

	pub fn add_block_tag(&self, block_tag: &com::github::javaparser::javadoc::javadoc_block_tag::JavadocBlockTag) -> com::github::javaparser::javadoc::javadoc::Javadoc {
		self.blockTags.add(block_tag);
		return self;
	}

	pub fn add_block_tag(&self, tag_name: &/* Java */ java::lang::String /**/, content: &/* Java */ java::lang::String /**/) -> com::github::javaparser::javadoc::javadoc::Javadoc {
		return self.add_block_tag(JavadocBlockTag::new(tag_name, content));
	}

	pub fn add_block_tag(&self, tag_name: &/* Java */ java::lang::String /**/, parameter: &/* Java */ java::lang::String /**/, content: &/* Java */ java::lang::String /**/) -> com::github::javaparser::javadoc::javadoc::Javadoc {
		return self.add_block_tag(tag_name, parameter + " " + content);
	}

	pub fn add_block_tag(&self, tag_name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::javadoc::javadoc::Javadoc {
		return self.add_block_tag(tag_name, "");
	}

	pub fn to_text(&self) -> /* Java */ java::lang::String /**/ {
		let sb: StringBuilder = StringBuilder::new();
		if !self.description.is_empty() {
			sb.append(&self.description.to_text());
			sb.append(LineSeparator::SYSTEM);
		}
		if !self.block_tags.isEmpty() {
			sb.append(LineSeparator::SYSTEM);
		}
		self.block_tags.forEach(|bt|{
			sb.append(&bt.to_text());
			sb.append(LineSeparator::SYSTEM);
		});
		return sb.toString();
	}

	pub fn to_comment(&self) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::comments::javadoc_comment::JavadocComment {
		return self.to_comment("")?;
	}

	pub fn to_comment(&self, indentation: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::comments::javadoc_comment::JavadocComment {
		for c in indentation.toCharArray() {
			if !Character::isWhitespace(c) {
				return Err(IllegalArgumentException::new("The indentation string should be composed only by whitespace characters"));
			}
		}
		let sb: StringBuilder = StringBuilder::new();
		sb.append(LineSeparator::SYSTEM);
		/* final */ let text: String = self.to_text();
		let comment_prefix: String =  if self.is_markdown_comment { "/// " } else { " * " };
		if !text.isEmpty() {
			for line in text.split(&LineSeparator::SYSTEM.as_raw_string()) {
				sb.append(indentation);
				sb.append(comment_prefix);
				sb.append(line);
				sb.append(LineSeparator::SYSTEM);
			}
		}
		if self.is_markdown_comment {
			return MarkdownComment::new(&sb.toString());
		} else {
			sb.append(indentation);
			sb.append(" ");
			return TraditionalJavadocComment::new(&sb.toString());
		}
	}

	pub fn get_description(&self) -> com::github::javaparser::javadoc::description::javadoc_description::JavadocDescription {
		return self.description;
	}

	pub fn get_block_tags(&self) -> /* Java */ java::util::List /**/ {
		return self.blockTags;
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if o == null || self.getClass() != o.getClass() {
			return false;
		}
	
		let document: Javadoc = o as Javadoc;
		return self.description.equals(document.description) && self.block_tags.equals(document.blockTags);
	}

	pub fn hash_code(&self) -> i32 {
		let result: i32 = self.description.hash_code();
		result = 31 * result + self.block_tags.hashCode();
		return result;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "Javadoc{" + "description=" + self.description + ", blockTags=" + self.block_tags + '}';
	}
}