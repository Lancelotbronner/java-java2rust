use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithStatements;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::BlockStmtMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct BlockStmt {
	statements: com::github::javaparser::ast::node_list::NodeList,
}

impl BlockStmt {
	pub fn new() -> com::github::javaparser::ast::stmt::block_stmt::BlockStmt {
		this(null, NodeList<>::new());
	}

	pub fn new(statements: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::stmt::block_stmt::BlockStmt {
		this(null, statements);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, statements: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::stmt::block_stmt::BlockStmt {
		super(token_range);
		self.set_statements(statements);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_statements(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.statements;
	}

	pub fn set_statements(&mut self, statements: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::block_stmt::BlockStmt {
		com::github::javaparser::utils::utils::Utils::assert_not_null(statements)?;
		if statements == self.statements {
			return self;
		}
		self.notify_property_change(ObservableProperty::STATEMENTS, self.statements, statements);
		if self.statements != null {
			self.statements.set_parent_node(null);
		}
	
		self.statements = statements;
		self.set_as_parent_node_of(statements);
		return self;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.statements.size() {
				{
					if self.statements.get(i) == node {
						self.statements.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::stmt::block_stmt::BlockStmt {
		return self.accept(CloneVisitor::new(), null) as BlockStmt;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::block_stmt_meta_model::BlockStmtMetaModel {
		return JavaParserMetaModel::blockStmtMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.statements.size() {
				{
					if self.statements.get(i) == node {
						self.statements.set(i, replacement_node as Statement)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.replace(node, replacement_node)?;
	}

	pub fn is_block_stmt(&self) -> bool {
		return true;
	}

	pub fn as_block_stmt(&self) -> com::github::javaparser::ast::stmt::block_stmt::BlockStmt {
		return self;
	}

	pub fn if_block_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_block_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl com::github::javaparser::ast::node_types::node_with_statements::NodeWithStatements for BlockStmt {}

impl /* Java */ java::lang::Cloneable /**/ for BlockStmt {}

impl com::github::javaparser::has_parent_node::HasParentNode for BlockStmt {}

impl com::github::javaparser::ast::observer::observable::Observable for BlockStmt {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for BlockStmt {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for BlockStmt {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for BlockStmt {}