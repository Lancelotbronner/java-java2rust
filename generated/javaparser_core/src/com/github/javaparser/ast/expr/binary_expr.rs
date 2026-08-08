use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::BinaryExprMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::printer::Stringable;
use java::util::Optional;
use java::util::function::Consumer;

pub struct BinaryExpr {
	left: com::github::javaparser::ast::expr::expression::Expression,
	right: com::github::javaparser::ast::expr::expression::Expression,
	operator: com::github::javaparser::ast::expr::binary_expr::Operator,
}

impl BinaryExpr {
	pub fn new() -> com::github::javaparser::ast::expr::binary_expr::BinaryExpr {
		this(null, BooleanLiteralExpr::new(), BooleanLiteralExpr::new(), Operator::EQUALS);
	}

	pub fn new(left: &com::github::javaparser::ast::expr::expression::Expression, right: &com::github::javaparser::ast::expr::expression::Expression, operator: &com::github::javaparser::ast::expr::binary_expr::Operator) -> com::github::javaparser::ast::expr::binary_expr::BinaryExpr {
		this(null, left, right, operator);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, left: &com::github::javaparser::ast::expr::expression::Expression, right: &com::github::javaparser::ast::expr::expression::Expression, operator: &com::github::javaparser::ast::expr::binary_expr::Operator) -> com::github::javaparser::ast::expr::binary_expr::BinaryExpr {
		super(token_range);
		self.set_left(left);
		self.set_right(right);
		self.set_operator(operator);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_left(&self) -> com::github::javaparser::ast::expr::expression::Expression {
		return self.left;
	}

	pub fn get_operator(&self) -> com::github::javaparser::ast::expr::binary_expr::Operator {
		return self.operator;
	}

	pub fn get_right(&self) -> com::github::javaparser::ast::expr::expression::Expression {
		return self.right;
	}

	pub fn set_left(&mut self, left: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::binary_expr::BinaryExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(left)?;
		if left == self.left {
			return self;
		}
		self.notify_property_change(ObservableProperty::LEFT, self.left, left);
		if self.left != null {
			self.left.set_parent_node(null);
		}
	
		self.left = left;
		self.set_as_parent_node_of(left);
		return self;
	}

	pub fn set_operator(&mut self, operator: &com::github::javaparser::ast::expr::binary_expr::Operator) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::binary_expr::BinaryExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(operator)?;
		if operator == self.operator {
			return self;
		}
		self.notify_property_change(ObservableProperty::OPERATOR, self.operator, operator);
		self.operator = operator;
		return self;
	}

	pub fn set_right(&mut self, right: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::binary_expr::BinaryExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(right)?;
		if right == self.right {
			return self;
		}
		self.notify_property_change(ObservableProperty::RIGHT, self.right, right);
		if self.right != null {
			self.right.set_parent_node(null);
		}
	
		self.right = right;
		self.set_as_parent_node_of(right);
		return self;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::binary_expr::BinaryExpr {
		return self.accept(CloneVisitor::new(), null) as BinaryExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::binary_expr_meta_model::BinaryExprMetaModel {
		return JavaParserMetaModel::binaryExprMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.left {
			self.set_left(replacement_node as Expression)?;
			return true;
		}
		if node == self.right {
			self.set_right(replacement_node as Expression)?;
			return true;
		}
		return super.replace(node, replacement_node);
	}

	pub fn is_binary_expr(&self) -> bool {
		return true;
	}

	pub fn as_binary_expr(&self) -> com::github::javaparser::ast::expr::binary_expr::BinaryExpr {
		return self;
	}

	pub fn if_binary_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_binary_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for BinaryExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for BinaryExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for BinaryExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for BinaryExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for BinaryExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for BinaryExpr {}

pub enum Operator {
	code_representation: /* Java */ java::lang::String /**/,
}