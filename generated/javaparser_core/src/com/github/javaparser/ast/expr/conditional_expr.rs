use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithCondition;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::ConditionalExprMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct ConditionalExpr {
	condition: com::github::javaparser::ast::expr::expression::Expression,
	then_expr: com::github::javaparser::ast::expr::expression::Expression,
	else_expr: com::github::javaparser::ast::expr::expression::Expression,
}

impl ConditionalExpr {
	pub fn new() -> com::github::javaparser::ast::expr::conditional_expr::ConditionalExpr {
		this(null, BooleanLiteralExpr::new(), StringLiteralExpr::new(), StringLiteralExpr::new());
	}

	pub fn new(condition: &com::github::javaparser::ast::expr::expression::Expression, then_expr: &com::github::javaparser::ast::expr::expression::Expression, else_expr: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::expr::conditional_expr::ConditionalExpr {
		this(null, condition, then_expr, else_expr);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, condition: &com::github::javaparser::ast::expr::expression::Expression, then_expr: &com::github::javaparser::ast::expr::expression::Expression, else_expr: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::expr::conditional_expr::ConditionalExpr {
		super(token_range);
		self.set_condition(condition);
		self.set_then_expr(then_expr);
		self.set_else_expr(else_expr);
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

	pub fn get_else_expr(&self) -> com::github::javaparser::ast::expr::expression::Expression {
		return self.else_expr;
	}

	pub fn get_then_expr(&self) -> com::github::javaparser::ast::expr::expression::Expression {
		return self.then_expr;
	}

	pub fn set_condition(&mut self, condition: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::conditional_expr::ConditionalExpr {
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

	pub fn set_else_expr(&mut self, else_expr: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::conditional_expr::ConditionalExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(else_expr)?;
		if else_expr == self.elseExpr {
			return self;
		}
		self.notify_property_change(ObservableProperty::ELSE_EXPR, self.elseExpr, else_expr);
		if self.elseExpr != null {
			self.elseExpr.set_parent_node(null);
		}
	
		self.elseExpr = else_expr;
		self.set_as_parent_node_of(else_expr);
		return self;
	}

	pub fn set_then_expr(&mut self, then_expr: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::conditional_expr::ConditionalExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(then_expr)?;
		if then_expr == self.thenExpr {
			return self;
		}
		self.notify_property_change(ObservableProperty::THEN_EXPR, self.thenExpr, then_expr);
		if self.thenExpr != null {
			self.thenExpr.set_parent_node(null);
		}
	
		self.thenExpr = then_expr;
		self.set_as_parent_node_of(then_expr);
		return self;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::conditional_expr::ConditionalExpr {
		return self.accept(CloneVisitor::new(), null) as ConditionalExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::conditional_expr_meta_model::ConditionalExprMetaModel {
		return JavaParserMetaModel::conditionalExprMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.condition {
			self.set_condition(replacement_node as Expression)?;
			return true;
		}
		if node == self.else_expr {
			self.set_else_expr(replacement_node as Expression)?;
			return true;
		}
		if node == self.then_expr {
			self.set_then_expr(replacement_node as Expression)?;
			return true;
		}
		return super.replace(node, replacement_node);
	}

	pub fn is_conditional_expr(&self) -> bool {
		return true;
	}

	pub fn as_conditional_expr(&self) -> com::github::javaparser::ast::expr::conditional_expr::ConditionalExpr {
		return self;
	}

	pub fn if_conditional_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_conditional_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn is_poly_expression(&self) -> bool {
		return self.appears_in_assignment_context() || self.appears_in_invocation_context();
	}
}

impl com::github::javaparser::ast::node_types::node_with_condition::NodeWithCondition for ConditionalExpr {}

impl /* Java */ java::lang::Cloneable /**/ for ConditionalExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for ConditionalExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for ConditionalExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for ConditionalExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for ConditionalExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for ConditionalExpr {}