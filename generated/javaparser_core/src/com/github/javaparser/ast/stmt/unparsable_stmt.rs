use crate::com::github::javaparser::ast::Node::Parsedness::UNPARSABLE;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::UnparsableStmtMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct UnparsableStmt;

impl UnparsableStmt {
	pub fn new() -> com::github::javaparser::ast::stmt::unparsable_stmt::UnparsableStmt {
		this(null);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange) -> com::github::javaparser::ast::stmt::unparsable_stmt::UnparsableStmt {
		super(token_range);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::stmt::unparsable_stmt::UnparsableStmt {
		return self.accept(CloneVisitor::new(), null) as UnparsableStmt;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::unparsable_stmt_meta_model::UnparsableStmtMetaModel {
		return JavaParserMetaModel::unparsableStmtMetaModel;
	}

	pub fn get_parsed(&self) -> com::github::javaparser::ast::node::Parsedness {
		return UNPARSABLE;
	}

	pub fn is_unparsable_stmt(&self) -> bool {
		return true;
	}

	pub fn as_unparsable_stmt(&self) -> com::github::javaparser::ast::stmt::unparsable_stmt::UnparsableStmt {
		return self;
	}

	pub fn if_unparsable_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_unparsable_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for UnparsableStmt {}

impl com::github::javaparser::has_parent_node::HasParentNode for UnparsableStmt {}

impl com::github::javaparser::ast::observer::observable::Observable for UnparsableStmt {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for UnparsableStmt {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for UnparsableStmt {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for UnparsableStmt {}