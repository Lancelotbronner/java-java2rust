use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::AssignExprMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::printer::Stringable;
use java::util::Optional;
use java::util::function::Consumer;

pub struct AssignExpr {
	target: com::github::javaparser::ast::expr::expression::Expression,
	value: com::github::javaparser::ast::expr::expression::Expression,
	operator: com::github::javaparser::ast::expr::assign_expr::Operator,
}

impl AssignExpr {
	pub fn new() -> com::github::javaparser::ast::expr::assign_expr::AssignExpr {
		this(null, NameExpr::new(), StringLiteralExpr::new(), Operator::ASSIGN);
	}

	pub fn new(target: &com::github::javaparser::ast::expr::expression::Expression, value: &com::github::javaparser::ast::expr::expression::Expression, operator: &com::github::javaparser::ast::expr::assign_expr::Operator) -> com::github::javaparser::ast::expr::assign_expr::AssignExpr {
		this(null, target, value, operator);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, target: &com::github::javaparser::ast::expr::expression::Expression, value: &com::github::javaparser::ast::expr::expression::Expression, operator: &com::github::javaparser::ast::expr::assign_expr::Operator) -> com::github::javaparser::ast::expr::assign_expr::AssignExpr {
		super(token_range);
		self.set_target(target);
		self.set_value(value);
		self.set_operator(operator);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_operator(&self) -> com::github::javaparser::ast::expr::assign_expr::Operator {
		return self.operator;
	}

	pub fn get_target(&self) -> com::github::javaparser::ast::expr::expression::Expression {
		return self.target;
	}

	pub fn get_value(&self) -> com::github::javaparser::ast::expr::expression::Expression {
		return self.value;
	}

	pub fn set_operator(&mut self, operator: &com::github::javaparser::ast::expr::assign_expr::Operator) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::assign_expr::AssignExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(operator)?;
		if operator == self.operator {
			return self;
		}
		self.notify_property_change(ObservableProperty::OPERATOR, self.operator, operator);
		self.operator = operator;
		return self;
	}

	pub fn set_target(&mut self, target: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::assign_expr::AssignExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(target)?;
		if target == self.target {
			return self;
		}
		self.notify_property_change(ObservableProperty::TARGET, self.target, target);
		if self.target != null {
			self.target.set_parent_node(null);
		}
	
		self.target = target;
		self.set_as_parent_node_of(target);
		return self;
	}

	pub fn set_value(&mut self, value: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::assign_expr::AssignExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(value)?;
		if value == self.value {
			return self;
		}
		self.notify_property_change(ObservableProperty::VALUE, self.value, value);
		if self.value != null {
			self.value.set_parent_node(null);
		}
	
		self.value = value;
		self.set_as_parent_node_of(value);
		return self;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::assign_expr::AssignExpr {
		return self.accept(CloneVisitor::new(), null) as AssignExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::assign_expr_meta_model::AssignExprMetaModel {
		return JavaParserMetaModel::assignExprMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.target {
			self.set_target(replacement_node as Expression)?;
			return true;
		}
		if node == self.value {
			self.set_value(replacement_node as Expression)?;
			return true;
		}
		return super.replace(node, replacement_node);
	}

	pub fn is_assign_expr(&self) -> bool {
		return true;
	}

	pub fn as_assign_expr(&self) -> com::github::javaparser::ast::expr::assign_expr::AssignExpr {
		return self;
	}

	pub fn if_assign_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_assign_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	fn is_assignment_context(&self) -> bool {
		return true;
	}
}

impl /* Java */ java::lang::Cloneable /**/ for AssignExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for AssignExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for AssignExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for AssignExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for AssignExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for AssignExpr {}

pub enum Operator {
	code_representation: /* Java */ java::lang::String /**/,
}