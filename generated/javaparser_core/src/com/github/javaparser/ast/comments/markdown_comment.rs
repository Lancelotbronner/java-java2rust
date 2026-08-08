use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::MarkdownCommentMetaModel;
use crate::com::github::javaparser::utils::LineSeparator;
use java::util::ArrayList;
use java::util::Arrays;
use java::util::Optional;
use java::util::function::Consumer;
use java::util::regex::Matcher;
use java::util::regex::Pattern;

pub struct MarkdownComment;

impl MarkdownComment {
	static markdownLinePattern: /* Java */ java::util::regex::Pattern /**/ = Pattern::compile("^\\s*///(.*)$");

	pub fn new() -> com::github::javaparser::ast::comments::markdown_comment::MarkdownComment {
		this(null, "empty");
	}

	pub fn new(content: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::comments::markdown_comment::MarkdownComment {
		this(null, content);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, content: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::comments::markdown_comment::MarkdownComment {
		super(token_range, content);
		self.custom_initialization();
	}

	pub fn get_markdown_content(&self) -> /* Java */ java::lang::String /**/ {
		let content: String = self.get_content();
		// Start by isolating the lines to make calculating and stripping leading whitespace easier
		let comment_lines: ArrayList<String> = ArrayList<>::new();
		comment_lines.addAll(&Arrays::asList(&content.split("(\r\n|\r|\n)")));
		let formatted_lines: ArrayList<String> = ArrayList<>::new();
		for line in comment_lines {
			// Use pattern matching to strip leading whitespace followed by /// for each of the lines.
			let matcher: Matcher = self.markdown_line_pattern.matcher(line);
			if matcher.matches() {
				formatted_lines.add(&matcher.group(1));
			} else {
				formatted_lines.add(line);
			}
		}
		// Find the length of the shortest whitespace prefix for all the lines so that this can be stripped according
		// to the Java specification. For example, treating . as whitespace in the example below, 2 spaces will be
		// stripped:
		// ///....prefix_length=4
		// ///......prefix_length=8
		// ///..prefix_length=2
		let shortest_whitespace_prefix: i32 = Integer::MAX_VALUE;
		for line in formatted_lines {
			 {
				let i: i32 = 0;
				while i < line.length() {
					{
						if !Character::isWhitespace(&line.charAt(i)) {
							shortest_whitespace_prefix = Math::min(shortest_whitespace_prefix, i);
							break;
						}
					}
					i += 1;
				 }
			 }
	
		}
		let content_builder: StringBuilder = StringBuilder::new();
		let line_separator: LineSeparator = LineSeparator::detect(content);
		// pattern match above.
		 {
			let i: i32 = 0;
			while i < formatted_lines.size() {
				{
					let line: String = formatted_lines.get(i);
					if line.trim().isEmpty() {
						content_builder.append(line);
					} else {
						content_builder.append(&line.substring(shortest_whitespace_prefix));
					}
					if i != formatted_lines.size() - 1 {
						content_builder.append(&line_separator.as_raw_string());
					}
				}
				i += 1;
			 }
		 }
	
		return content_builder.toString();
	}

	pub fn get_header(&self) -> /* Java */ java::lang::String /**/ {
		return "";
	}

	pub fn get_footer(&self) -> /* Java */ java::lang::String /**/ {
		return "";
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn as_string(&self) -> /* Java */ java::lang::String /**/ {
		let content: String = self.get_content();
		// Try to preserve line separators
		let line_separator: String = self.get_line_ending_style().as_raw_string();
		let lines: Vec<String> = content.split(line_separator);
		let builder: StringBuilder = StringBuilder::new();
		for line in lines {
			builder.append(&self.get_header());
			builder.append(line);
			builder.append(line_separator);
		}
		return builder.toString();
	}

	pub fn is_markdown_comment(&self) -> bool {
		return true;
	}

	pub fn as_markdown_comment(&self) -> com::github::javaparser::ast::comments::markdown_comment::MarkdownComment {
		return self;
	}

	pub fn to_markdown_comment(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn if_markdown_comment(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::comments::markdown_comment::MarkdownComment {
		return self.accept(CloneVisitor::new(), null) as MarkdownComment;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::markdown_comment_meta_model::MarkdownCommentMetaModel {
		return JavaParserMetaModel::markdownCommentMetaModel;
	}
}

impl /* Java */ java::lang::Cloneable /**/ for MarkdownComment {}

impl com::github::javaparser::has_parent_node::HasParentNode for MarkdownComment {}

impl com::github::javaparser::ast::observer::observable::Observable for MarkdownComment {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for MarkdownComment {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for MarkdownComment {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for MarkdownComment {}