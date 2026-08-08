use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::expr::BooleanLiteralExpr;
use crate::com::github::javaparser::ast::expr::Expression;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithCondition;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::DerivedProperty;
use crate::com::github::javaparser::metamodel::IfStmtMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::OptionalProperty;
use java::util::Optional;
use java::util::function::Consumer;

pub struct IfStmt {
	condition: com::github::javaparser::ast::expr::expression::Expression,
	then_stmt: com::github::javaparser::ast::stmt::statement::Statement,
	else_stmt: com::github::javaparser::ast::stmt::statement::Statement,
}

impl IfStmt {
	pub fn new() -> com::github::javaparser::ast::stmt::if_stmt::IfStmt {
		this(null, BooleanLiteralExpr::new(), ReturnStmt::new(), null);
	}

	pub fn new(condition: &com::github::javaparser::ast::expr::expression::Expression, then_stmt: &com::github::javaparser::ast::stmt::statement::Statement, else_stmt: &com::github::javaparser::ast::stmt::statement::Statement) -> com::github::javaparser::ast::stmt::if_stmt::IfStmt {
		this(null, condition, then_stmt, else_stmt);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, condition: &com::github::javaparser::ast::expr::expression::Expression, then_stmt: &com::github::javaparser::ast::stmt::statement::Statement, else_stmt: &com::github::javaparser::ast::stmt::statement::Statement) -> com::github::javaparser::ast::stmt::if_stmt::IfStmt {
		super(token_range);
		self.set_condition(condition);
		self.set_then_stmt(then_stmt);
		self.set_else_stmt(else_stmt);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_condition(&self) -> com::github::javaparser::ast::expr::expression::Expression {
		return self.condition;
	}

	pub fn get_else_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.else_stmt);
	}

	pub fn get_then_stmt(&self) -> com::github::javaparser::ast::stmt::statement::Statement {
		return self.then_stmt;
	}

	pub fn set_condition(&mut self, condition: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::if_stmt::IfStmt {
		com::github::javaparser::utils::utils::Utils::assert_not_null(condition)?;
		if condition == self.condition {
			return self;
		}
		self.notify_property_change(ObservableProperty::CONDITION, self.condition, condition);
		if self.condition != null {
			self.condition.set_parent_node(null);
		}
	
		self.condition = condition;
		self.set_as_parent_node_of(condition);
		return self;
	}

	pub fn set_else_stmt(&mut self, else_stmt: &com::github::javaparser::ast::stmt::statement::Statement) -> com::github::javaparser::ast::stmt::if_stmt::IfStmt {
		if else_stmt == self.elseStmt {
			return self;
		}
		self.notify_property_change(ObservableProperty::ELSE_STMT, self.elseStmt, else_stmt);
		if self.elseStmt != null {
			self.elseStmt.set_parent_node(null);
		}
	
		self.elseStmt = else_stmt;
		self.set_as_parent_node_of(else_stmt);
		return self;
	}

	pub fn set_then_stmt(&mut self, then_stmt: &com::github::javaparser::ast::stmt::statement::Statement) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::if_stmt::IfStmt {
		com::github::javaparser::utils::utils::Utils::assert_not_null(then_stmt)?;
		if then_stmt == self.thenStmt {
			return self;
		}
		self.notify_property_change(ObservableProperty::THEN_STMT, self.thenStmt, then_stmt);
		if self.thenStmt != null {
			self.thenStmt.set_parent_node(null);
		}
	
		self.thenStmt = then_stmt;
		self.set_as_parent_node_of(then_stmt);
		return self;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		if self.else_stmt != null {
			if node == self.else_stmt {
				self.remove_else_stmt();
				return true;
			}
		}
		return super.remove(node);
	}

	pub fn remove_else_stmt(&self) -> com::github::javaparser::ast::stmt::if_stmt::IfStmt {
		return self.set_else_stmt(null as Statement);
	}

	pub fn has_then_block(&self) -> bool {
		return self.then_stmt instanceof BlockStmt;
	}

	pub fn has_else_block(&self) -> bool {
		return self.else_stmt instanceof BlockStmt;
	}

	pub fn has_cascading_if_stmt(&self) -> bool {
		return self.else_stmt instanceof IfStmt;
	}

	pub fn has_else_branch(&self) -> bool {
		return self.else_stmt != null;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::stmt::if_stmt::IfStmt {
		return self.accept(CloneVisitor::new(), null) as IfStmt;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::if_stmt_meta_model::IfStmtMetaModel {
		return JavaParserMetaModel::ifStmtMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.condition {
			self.set_condition(replacement_node as Expression)?;
			return true;
		}
		if self.else_stmt != null {
			if node == self.else_stmt {
				self.set_else_stmt(replacement_node as Statement);
				return true;
			}
		}
		if node == self.then_stmt {
			self.set_then_stmt(replacement_node as Statement)?;
			return true;
		}
		return super.replace(node, replacement_node)?;
	}

	pub fn is_if_stmt(&self) -> bool {
		return true;
	}

	pub fn as_if_stmt(&self) -> com::github::javaparser::ast::stmt::if_stmt::IfStmt {
		return self;
	}

	pub fn if_if_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_if_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl com::github::javaparser::ast::node_types::node_with_condition::NodeWithCondition for IfStmt {}

impl /* Java */ java::lang::Cloneable /**/ for IfStmt {}

impl com::github::javaparser::has_parent_node::HasParentNode for IfStmt {}

impl com::github::javaparser::ast::observer::observable::Observable for IfStmt {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for IfStmt {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for IfStmt {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for IfStmt {}