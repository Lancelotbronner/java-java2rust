use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::expr::Expression;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::OptionalProperty;
use crate::com::github::javaparser::metamodel::TryStmtMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct TryStmt {
	resources: com::github::javaparser::ast::node_list::NodeList,
	try_block: com::github::javaparser::ast::stmt::block_stmt::BlockStmt,
	catch_clauses: com::github::javaparser::ast::node_list::NodeList,
	finally_block: com::github::javaparser::ast::stmt::block_stmt::BlockStmt,
}

impl TryStmt {
	pub fn new() -> com::github::javaparser::ast::stmt::try_stmt::TryStmt {
		this(null, NodeList<>::new(), BlockStmt::new(), NodeList<>::new(), null);
	}

	pub fn new(try_block: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt, catch_clauses: &com::github::javaparser::ast::node_list::NodeList, finally_block: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt) -> com::github::javaparser::ast::stmt::try_stmt::TryStmt {
		this(null, NodeList<>::new(), try_block, catch_clauses, finally_block);
	}

	pub fn new(resources: &com::github::javaparser::ast::node_list::NodeList, try_block: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt, catch_clauses: &com::github::javaparser::ast::node_list::NodeList, finally_block: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt) -> com::github::javaparser::ast::stmt::try_stmt::TryStmt {
		this(null, resources, try_block, catch_clauses, finally_block);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, resources: &com::github::javaparser::ast::node_list::NodeList, try_block: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt, catch_clauses: &com::github::javaparser::ast::node_list::NodeList, finally_block: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt) -> com::github::javaparser::ast::stmt::try_stmt::TryStmt {
		super(token_range);
		self.set_resources(resources);
		self.set_try_block(try_block);
		self.set_catch_clauses(catch_clauses);
		self.set_finally_block(finally_block);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_catch_clauses(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.catch_clauses;
	}

	pub fn get_finally_block(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.finally_block);
	}

	pub fn get_try_block(&self) -> com::github::javaparser::ast::stmt::block_stmt::BlockStmt {
		return self.try_block;
	}

	pub fn get_resources(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.resources;
	}

	pub fn set_catch_clauses(&mut self, catch_clauses: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::try_stmt::TryStmt {
		com::github::javaparser::utils::utils::Utils::assert_not_null(catch_clauses)?;
		if catch_clauses == self.catchClauses {
			return self;
		}
		self.notify_property_change(ObservableProperty::CATCH_CLAUSES, self.catchClauses, catch_clauses);
		if self.catchClauses != null {
			self.catchClauses.set_parent_node(null);
		}
	
		self.catchClauses = catch_clauses;
		self.set_as_parent_node_of(catch_clauses);
		return self;
	}

	pub fn set_finally_block(&mut self, finally_block: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt) -> com::github::javaparser::ast::stmt::try_stmt::TryStmt {
		if finally_block == self.finallyBlock {
			return self;
		}
		self.notify_property_change(ObservableProperty::FINALLY_BLOCK, self.finallyBlock, finally_block);
		if self.finallyBlock != null {
			self.finallyBlock.set_parent_node(null);
		}
	
		self.finallyBlock = finally_block;
		self.set_as_parent_node_of(finally_block);
		return self;
	}

	pub fn set_try_block(&mut self, try_block: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::try_stmt::TryStmt {
		com::github::javaparser::utils::utils::Utils::assert_not_null(try_block)?;
		if try_block == self.tryBlock {
			return self;
		}
		self.notify_property_change(ObservableProperty::TRY_BLOCK, self.tryBlock, try_block);
		if self.tryBlock != null {
			self.tryBlock.set_parent_node(null);
		}
	
		self.tryBlock = try_block;
		self.set_as_parent_node_of(try_block);
		return self;
	}

	pub fn set_resources(&mut self, resources: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::try_stmt::TryStmt {
		com::github::javaparser::utils::utils::Utils::assert_not_null(resources)?;
		if resources == self.resources {
			return self;
		}
		self.notify_property_change(ObservableProperty::RESOURCES, self.resources, resources);
		if self.resources != null {
			self.resources.set_parent_node(null);
		}
	
		self.resources = resources;
		self.set_as_parent_node_of(resources);
		return self;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.catch_clauses.size() {
				{
					if self.catch_clauses.get(i) == node {
						self.catch_clauses.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		if self.finally_block != null {
			if node == self.finally_block {
				self.remove_finally_block();
				return true;
			}
		}
		 {
			let i: i32 = 0;
			while i < self.resources.size() {
				{
					if self.resources.get(i) == node {
						self.resources.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn remove_finally_block(&self) -> com::github::javaparser::ast::stmt::try_stmt::TryStmt {
		return self.set_finally_block(null as BlockStmt);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::stmt::try_stmt::TryStmt {
		return self.accept(CloneVisitor::new(), null) as TryStmt;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::try_stmt_meta_model::TryStmtMetaModel {
		return JavaParserMetaModel::tryStmtMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.catch_clauses.size() {
				{
					if self.catch_clauses.get(i) == node {
						self.catch_clauses.set(i, replacement_node as CatchClause)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		if self.finally_block != null {
			if node == self.finally_block {
				self.set_finally_block(replacement_node as BlockStmt);
				return true;
			}
		}
		 {
			let i: i32 = 0;
			while i < self.resources.size() {
				{
					if self.resources.get(i) == node {
						self.resources.set(i, replacement_node as Expression)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		if node == self.try_block {
			self.set_try_block(replacement_node as BlockStmt)?;
			return true;
		}
		return super.replace(node, replacement_node)?;
	}

	pub fn is_try_stmt(&self) -> bool {
		return true;
	}

	pub fn as_try_stmt(&self) -> com::github::javaparser::ast::stmt::try_stmt::TryStmt {
		return self;
	}

	pub fn if_try_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_try_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for TryStmt {}

impl com::github::javaparser::has_parent_node::HasParentNode for TryStmt {}

impl com::github::javaparser::ast::observer::observable::Observable for TryStmt {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for TryStmt {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for TryStmt {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for TryStmt {}