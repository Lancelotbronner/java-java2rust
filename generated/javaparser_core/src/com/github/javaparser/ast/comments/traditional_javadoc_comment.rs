use crate::com::github::javaparser::StaticJavaParser::parseJavadoc;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::javadoc::Javadoc;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::TraditionalJavadocCommentMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct TraditionalJavadocComment;

impl TraditionalJavadocComment {
	pub fn new() -> com::github::javaparser::ast::comments::traditional_javadoc_comment::TraditionalJavadocComment {
		this(null, "empty");
	}

	pub fn new(content: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::comments::traditional_javadoc_comment::TraditionalJavadocComment {
		this(null, content);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, content: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::comments::traditional_javadoc_comment::TraditionalJavadocComment {
		super(token_range, content);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn parse(&self) -> com::github::javaparser::javadoc::javadoc::Javadoc {
		return com::github::javaparser::static_java_parser::StaticJavaParser::parse_javadoc(&self.get_content(), false);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::comments::traditional_javadoc_comment::TraditionalJavadocComment {
		return self.accept(CloneVisitor::new(), null) as TraditionalJavadocComment;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::traditional_javadoc_comment_meta_model::TraditionalJavadocCommentMetaModel {
		return JavaParserMetaModel::traditionalJavadocCommentMetaModel;
	}

	pub fn is_traditional_javadoc_comment(&self) -> bool {
		return true;
	}

	pub fn as_traditional_javadoc_comment(&self) -> com::github::javaparser::ast::comments::traditional_javadoc_comment::TraditionalJavadocComment {
		return self;
	}

	pub fn if_traditional_javadoc_comment(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_traditional_javadoc_comment(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn get_header(&self) -> /* Java */ java::lang::String /**/ {
		return "/**";
	}

	pub fn get_footer(&self) -> /* Java */ java::lang::String /**/ {
		return "*/";
	}
}

impl /* Java */ java::lang::Cloneable /**/ for TraditionalJavadocComment {}

impl com::github::javaparser::has_parent_node::HasParentNode for TraditionalJavadocComment {}

impl com::github::javaparser::ast::observer::observable::Observable for TraditionalJavadocComment {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for TraditionalJavadocComment {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for TraditionalJavadocComment {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for TraditionalJavadocComment {}