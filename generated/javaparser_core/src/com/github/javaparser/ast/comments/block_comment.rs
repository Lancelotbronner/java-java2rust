use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::BlockCommentMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct BlockComment;

impl BlockComment {
	pub fn new() -> com::github::javaparser::ast::comments::block_comment::BlockComment {
		this(null, "empty");
	}

	pub fn new(content: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::comments::block_comment::BlockComment {
		this(null, content);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, content: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::comments::block_comment::BlockComment {
		super(token_range, content);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::comments::block_comment::BlockComment {
		return self.accept(CloneVisitor::new(), null) as BlockComment;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::block_comment_meta_model::BlockCommentMetaModel {
		return JavaParserMetaModel::blockCommentMetaModel;
	}

	pub fn is_block_comment(&self) -> bool {
		return true;
	}

	pub fn as_block_comment(&self) -> com::github::javaparser::ast::comments::block_comment::BlockComment {
		return self;
	}

	pub fn if_block_comment(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_block_comment(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn get_header(&self) -> /* Java */ java::lang::String /**/ {
		return "/*";
	}

	pub fn get_footer(&self) -> /* Java */ java::lang::String /**/ {
		return "*/";
	}
}

impl /* Java */ java::lang::Cloneable /**/ for BlockComment {}

impl com::github::javaparser::has_parent_node::HasParentNode for BlockComment {}

impl com::github::javaparser::ast::observer::observable::Observable for BlockComment {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for BlockComment {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for BlockComment {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for BlockComment {}