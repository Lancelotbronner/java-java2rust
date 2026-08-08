use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithExpression;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithType;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::type::ClassOrInterfaceType;
use crate::com::github::javaparser::ast::type::ReferenceType;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::InstanceOfExprMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::OptionalProperty;
use java::util::Optional;
use java::util::function::Consumer;

pub struct InstanceOfExpr {
	expression: com::github::javaparser::ast::expr::expression::Expression,
	pattern: com::github::javaparser::ast::expr::pattern_expr::PatternExpr,
	type: com::github::javaparser::ast::type::reference_type::ReferenceType,
}

impl InstanceOfExpr {
	pub fn new() -> com::github::javaparser::ast::expr::instance_of_expr::InstanceOfExpr {
		this(null, NameExpr::new(), ClassOrInterfaceType::new(), null);
	}

	pub fn new(expression: &com::github::javaparser::ast::expr::expression::Expression, type: &com::github::javaparser::ast::type::reference_type::ReferenceType) -> com::github::javaparser::ast::expr::instance_of_expr::InstanceOfExpr {
		this(null, expression, type, null);
	}

	pub fn new(expression: &com::github::javaparser::ast::expr::expression::Expression, type: &com::github::javaparser::ast::type::reference_type::ReferenceType, pattern: &com::github::javaparser::ast::expr::pattern_expr::PatternExpr) -> com::github::javaparser::ast::expr::instance_of_expr::InstanceOfExpr {
		this(null, expression, type, pattern);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, expression: &com::github::javaparser::ast::expr::expression::Expression, type: &com::github::javaparser::ast::type::reference_type::ReferenceType, pattern: &com::github::javaparser::ast::expr::pattern_expr::PatternExpr) -> com::github::javaparser::ast::expr::instance_of_expr::InstanceOfExpr {
		super(token_range);
		self.set_expression(expression);
		self.set_type(type);
		self.set_pattern(pattern);
		self.custom_initialization();
	}

	pub fn get_name(&self) /* thrown(java.lang.IllegalStateException) */ -> /* Java */ java::util::Optional /**/ {
		if self.pattern == null {
			return Optional::empty();
		}
		if !self.pattern.is_type_pattern_expr() {
			return Optional::empty();
		}
		return Optional::of(&self.pattern.as_type_pattern_expr()?.get_name());
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn as_instance_of_expr(&self) -> com::github::javaparser::ast::expr::instance_of_expr::InstanceOfExpr {
		return self;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::instance_of_expr::InstanceOfExpr {
		return self.accept(CloneVisitor::new(), null) as InstanceOfExpr;
	}

	pub fn get_expression(&self) -> com::github::javaparser::ast::expr::expression::Expression {
		return self.expression;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::instance_of_expr_meta_model::InstanceOfExprMetaModel {
		return JavaParserMetaModel::instanceOfExprMetaModel;
	}

	pub fn get_pattern(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.pattern);
	}

	pub fn get_type(&self) -> com::github::javaparser::ast::type::reference_type::ReferenceType {
		return self.type;
	}

	pub fn if_instance_of_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn is_instance_of_expr(&self) -> bool {
		return true;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		if self.pattern != null {
			if node == self.pattern {
				self.remove_pattern();
				return true;
			}
		}
		return super.remove(node);
	}

	pub fn remove_pattern(&self) -> com::github::javaparser::ast::expr::instance_of_expr::InstanceOfExpr {
		return self.set_pattern(null as PatternExpr);
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		if node == self.expression {
			self.set_expression(replacement_node as Expression);
			return true;
		}
		if self.pattern != null {
			if node == self.pattern {
				self.set_pattern(replacement_node as PatternExpr);
				return true;
			}
		}
		if node == self.type {
			self.set_type(replacement_node as ReferenceType);
			return true;
		}
		return super.replace(node, replacement_node);
	}

	pub fn set_expression(&mut self, expression: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::instance_of_expr::InstanceOfExpr {
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

	pub fn set_pattern(&mut self, pattern: &com::github::javaparser::ast::expr::pattern_expr::PatternExpr) -> com::github::javaparser::ast::expr::instance_of_expr::InstanceOfExpr {
		if pattern == self.pattern {
			return self;
		}
		self.notify_property_change(ObservableProperty::PATTERN, self.pattern, pattern);
		if self.pattern != null {
			self.pattern.set_parent_node(null);
		}
	
		self.pattern = pattern;
		self.set_as_parent_node_of(pattern);
		return self;
	}

	pub fn set_type(&mut self, type: &com::github::javaparser::ast::type::reference_type::ReferenceType) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::instance_of_expr::InstanceOfExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(type)?;
		if type == self.type {
			return self;
		}
		self.notify_property_change(ObservableProperty::TYPE, self.type, type);
		if self.type != null {
			self.type.set_parent_node(null);
		}
	
		self.type = type;
		self.set_as_parent_node_of(type);
		return self;
	}

	pub fn to_instance_of_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl com::github::javaparser::ast::node_types::node_with_type::NodeWithType for InstanceOfExpr {}

impl com::github::javaparser::ast::node_types::node_with_expression::NodeWithExpression for InstanceOfExpr {}

impl /* Java */ java::lang::Cloneable /**/ for InstanceOfExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for InstanceOfExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for InstanceOfExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for InstanceOfExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for InstanceOfExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for InstanceOfExpr {}