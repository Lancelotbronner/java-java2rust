use crate::com::github::javaparser::StaticJavaParser::parseJavadoc;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::javadoc::Javadoc;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::JavadocCommentMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct JavadocComment;

impl JavadocComment {
	pub fn new() -> com::github::javaparser::ast::comments::javadoc_comment::JavadocComment {
		this(null, "empty");
	}

	pub fn new(content: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::comments::javadoc_comment::JavadocComment {
		this(null, content);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, content: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::comments::javadoc_comment::JavadocComment {
		super(token_range, content);
		self.custom_initialization();
	}

	pub fn is_javadoc_comment(&self) -> bool {
		return true;
	}

	pub fn as_javadoc_comment(&self) -> com::github::javaparser::ast::comments::javadoc_comment::JavadocComment {
		return self;
	}

	pub fn to_javadoc_comment(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn if_javadoc_comment(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::comments::javadoc_comment::JavadocComment {
		return self.accept(CloneVisitor::new(), null) as JavadocComment;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::javadoc_comment_meta_model::JavadocCommentMetaModel {
		return JavaParserMetaModel::javadocCommentMetaModel;
	}

	pub fn parse(&self) -> com::github::javaparser::javadoc::javadoc::Javadoc {
		return com::github::javaparser::static_java_parser::StaticJavaParser::parse_javadoc(&self.get_content(), &self.is_markdown_comment());
	}
}

impl /* Java */ java::lang::Cloneable /**/ for JavadocComment {}

impl com::github::javaparser::has_parent_node::HasParentNode for JavadocComment {}

impl com::github::javaparser::ast::observer::observable::Observable for JavadocComment {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for JavadocComment {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for JavadocComment {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for JavadocComment {}