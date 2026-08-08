use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::expr::Expression;
use crate::com::github::javaparser::ast::expr::NameExpr;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithExpression;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::YieldStmtMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct YieldStmt {
	expression: com::github::javaparser::ast::expr::expression::Expression,
}

impl YieldStmt {
	pub fn new() -> com::github::javaparser::ast::stmt::yield_stmt::YieldStmt {
		this(null, NameExpr::new());
	}

	pub fn new(expression: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::stmt::yield_stmt::YieldStmt {
		this(null, expression);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, expression: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::stmt::yield_stmt::YieldStmt {
		super(token_range);
		self.set_expression(expression);
		self.custom_initialization();
	}

	pub fn get_expression(&self) -> com::github::javaparser::ast::expr::expression::Expression {
		return self.expression;
	}

	pub fn set_expression(&mut self, expression: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::yield_stmt::YieldStmt {
		com::github::javaparser::utils::utils::Utils::assert_not_null(expression)?;
		if expression == self.expression {
			return self;
		}
		self.notify_property_change(ObservableProperty::EXPRESSION, self.expression, expression);
		if self.expression != null {
			self.expression.set_parent_node(null);
		}
	
		self.expression = expression;
		self.set_as_parent_node_of(expression);
		return self;
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn is_yield_stmt(&self) -> bool {
		return true;
	}

	pub fn as_yield_stmt(&self) -> com::github::javaparser::ast::stmt::yield_stmt::YieldStmt {
		return self;
	}

	pub fn to_yield_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn if_yield_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.expression {
			self.set_expression(replacement_node as Expression)?;
			return true;
		}
		return super.replace(node, replacement_node)?;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::stmt::yield_stmt::YieldStmt {
		return self.accept(CloneVisitor::new(), null) as YieldStmt;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::yield_stmt_meta_model::YieldStmtMetaModel {
		return JavaParserMetaModel::yieldStmtMetaModel;
	}
}

impl com::github::javaparser::ast::node_types::node_with_expression::NodeWithExpression for YieldStmt {}

impl /* Java */ java::lang::Cloneable /**/ for YieldStmt {}

impl com::github::javaparser::has_parent_node::HasParentNode for YieldStmt {}

impl com::github::javaparser::ast::observer::observable::Observable for YieldStmt {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for YieldStmt {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for YieldStmt {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for YieldStmt {}