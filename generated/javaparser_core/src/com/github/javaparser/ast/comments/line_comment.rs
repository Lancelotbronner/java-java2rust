use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::LineCommentMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct LineComment;

impl LineComment {
	pub fn new() -> com::github::javaparser::ast::comments::line_comment::LineComment {
		this(null, "empty");
	}

	pub fn new(content: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::comments::line_comment::LineComment {
		this(null, content);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, content: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::comments::line_comment::LineComment {
		super(token_range, content);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn is_line_comment(&self) -> bool {
		return true;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::comments::line_comment::LineComment {
		return self.accept(CloneVisitor::new(), null) as LineComment;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::line_comment_meta_model::LineCommentMetaModel {
		return JavaParserMetaModel::lineCommentMetaModel;
	}

	pub fn as_line_comment(&self) -> com::github::javaparser::ast::comments::line_comment::LineComment {
		return self;
	}

	pub fn if_line_comment(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_line_comment(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn get_header(&self) -> /* Java */ java::lang::String /**/ {
		return "//";
	}

	pub fn get_footer(&self) -> /* Java */ java::lang::String /**/ {
		return "";
	}
}

impl /* Java */ java::lang::Cloneable /**/ for LineComment {}

impl com::github::javaparser::has_parent_node::HasParentNode for LineComment {}

impl com::github::javaparser::ast::observer::observable::Observable for LineComment {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for LineComment {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for LineComment {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for LineComment {}