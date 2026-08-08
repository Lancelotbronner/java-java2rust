use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithExpression;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::DerivedProperty;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::UnaryExprMetaModel;
use crate::com::github::javaparser::printer::Stringable;
use java::util::Optional;
use java::util::function::Consumer;

pub struct UnaryExpr {
	expression: com::github::javaparser::ast::expr::expression::Expression,
	operator: com::github::javaparser::ast::expr::unary_expr::Operator,
}

impl UnaryExpr {
	pub fn new() -> com::github::javaparser::ast::expr::unary_expr::UnaryExpr {
		this(null, IntegerLiteralExpr::new(), Operator::POSTFIX_INCREMENT);
	}

	pub fn new(expression: &com::github::javaparser::ast::expr::expression::Expression, operator: &com::github::javaparser::ast::expr::unary_expr::Operator) -> com::github::javaparser::ast::expr::unary_expr::UnaryExpr {
		this(null, expression, operator);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, expression: &com::github::javaparser::ast::expr::expression::Expression, operator: &com::github::javaparser::ast::expr::unary_expr::Operator) -> com::github::javaparser::ast::expr::unary_expr::UnaryExpr {
		super(token_range);
		self.set_expression(expression);
		self.set_operator(operator);
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

	pub fn get_operator(&self) -> com::github::javaparser::ast::expr::unary_expr::Operator {
		return self.operator;
	}

	pub fn set_expression(&mut self, expression: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::unary_expr::UnaryExpr {
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

	pub fn set_operator(&mut self, operator: &com::github::javaparser::ast::expr::unary_expr::Operator) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::unary_expr::UnaryExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(operator)?;
		if operator == self.operator {
			return self;
		}
		self.notify_property_change(ObservableProperty::OPERATOR, self.operator, operator);
		self.operator = operator;
		return self;
	}

	pub fn is_postfix(&self) -> bool {
		return self.operator.is_postfix();
	}

	pub fn is_prefix(&self) -> bool {
		return !self.is_postfix();
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::unary_expr::UnaryExpr {
		return self.accept(CloneVisitor::new(), null) as UnaryExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::unary_expr_meta_model::UnaryExprMetaModel {
		return JavaParserMetaModel::unaryExprMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.expression {
			self.set_expression(replacement_node as Expression)?;
			return true;
		}
		return super.replace(node, replacement_node);
	}

	pub fn is_unary_expr(&self) -> bool {
		return true;
	}

	pub fn as_unary_expr(&self) -> com::github::javaparser::ast::expr::unary_expr::UnaryExpr {
		return self;
	}

	pub fn if_unary_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_unary_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl com::github::javaparser::ast::node_types::node_with_expression::NodeWithExpression for UnaryExpr {}

impl /* Java */ java::lang::Cloneable /**/ for UnaryExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for UnaryExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for UnaryExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for UnaryExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for UnaryExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for UnaryExpr {}

pub enum Operator {
	code_representation: /* Java */ java::lang::String /**/,
	is_postfix: bool,
}