use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::EmptyStmtMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct EmptyStmt;

impl EmptyStmt {
	pub fn new() -> com::github::javaparser::ast::stmt::empty_stmt::EmptyStmt {
		this(null);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange) -> com::github::javaparser::ast::stmt::empty_stmt::EmptyStmt {
		super(token_range);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::stmt::empty_stmt::EmptyStmt {
		return self.accept(CloneVisitor::new(), null) as EmptyStmt;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::empty_stmt_meta_model::EmptyStmtMetaModel {
		return JavaParserMetaModel::emptyStmtMetaModel;
	}

	pub fn is_empty_stmt(&self) -> bool {
		return true;
	}

	pub fn as_empty_stmt(&self) -> com::github::javaparser::ast::stmt::empty_stmt::EmptyStmt {
		return self;
	}

	pub fn if_empty_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_empty_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for EmptyStmt {}

impl com::github::javaparser::has_parent_node::HasParentNode for EmptyStmt {}

impl com::github::javaparser::ast::observer::observable::Observable for EmptyStmt {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for EmptyStmt {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for EmptyStmt {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for EmptyStmt {}