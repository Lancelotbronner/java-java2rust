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
use crate::com::github::javaparser::metamodel::ThrowStmtMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct ThrowStmt {
	expression: com::github::javaparser::ast::expr::expression::Expression,
}

impl ThrowStmt {
	pub fn new() -> com::github::javaparser::ast::stmt::throw_stmt::ThrowStmt {
		this(null, NameExpr::new());
	}

	pub fn new(expression: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::stmt::throw_stmt::ThrowStmt {
		this(null, expression);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, expression: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::stmt::throw_stmt::ThrowStmt {
		super(token_range);
		self.set_expression(expression);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_expression(&self) -> com::github::javaparser::ast::expr::expression::Expression {
		return self.expression;
	}

	pub fn set_expression(&mut self, expression: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::throw_stmt::ThrowStmt {
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

	pub fn clone(&self) -> com::github::javaparser::ast::stmt::throw_stmt::ThrowStmt {
		return self.accept(CloneVisitor::new(), null) as ThrowStmt;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::throw_stmt_meta_model::ThrowStmtMetaModel {
		return JavaParserMetaModel::throwStmtMetaModel;
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

	pub fn is_throw_stmt(&self) -> bool {
		return true;
	}

	pub fn as_throw_stmt(&self) -> com::github::javaparser::ast::stmt::throw_stmt::ThrowStmt {
		return self;
	}

	pub fn if_throw_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_throw_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl com::github::javaparser::ast::node_types::node_with_expression::NodeWithExpression for ThrowStmt {}

impl /* Java */ java::lang::Cloneable /**/ for ThrowStmt {}

impl com::github::javaparser::has_parent_node::HasParentNode for ThrowStmt {}

impl com::github::javaparser::ast::observer::observable::Observable for ThrowStmt {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for ThrowStmt {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for ThrowStmt {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for ThrowStmt {}