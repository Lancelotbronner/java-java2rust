use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithExpression;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithType;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::type::ClassOrInterfaceType;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::CastExprMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct CastExpr {
	type: com::github::javaparser::ast::type::type::Type,
	expression: com::github::javaparser::ast::expr::expression::Expression,
}

impl CastExpr {
	pub fn new() -> com::github::javaparser::ast::expr::cast_expr::CastExpr {
		this(null, ClassOrInterfaceType::new(), NameExpr::new());
	}

	pub fn new(type: &com::github::javaparser::ast::type::type::Type, expression: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::expr::cast_expr::CastExpr {
		this(null, type, expression);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, type: &com::github::javaparser::ast::type::type::Type, expression: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::expr::cast_expr::CastExpr {
		super(token_range);
		self.set_type(type);
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

	pub fn get_type(&self) -> com::github::javaparser::ast::type::type::Type {
		return self.type;
	}

	pub fn set_expression(&mut self, expression: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::cast_expr::CastExpr {
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

	pub fn set_type(&mut self, type: &com::github::javaparser::ast::type::type::Type) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::cast_expr::CastExpr {
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

	pub fn clone(&self) -> com::github::javaparser::ast::expr::cast_expr::CastExpr {
		return self.accept(CloneVisitor::new(), null) as CastExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::cast_expr_meta_model::CastExprMetaModel {
		return JavaParserMetaModel::castExprMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.expression {
			self.set_expression(replacement_node as Expression)?;
			return true;
		}
		if node == self.type {
			self.set_type(replacement_node as Type)?;
			return true;
		}
		return super.replace(node, replacement_node);
	}

	pub fn is_cast_expr(&self) -> bool {
		return true;
	}

	pub fn as_cast_expr(&self) -> com::github::javaparser::ast::expr::cast_expr::CastExpr {
		return self;
	}

	pub fn if_cast_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_cast_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl com::github::javaparser::ast::node_types::node_with_type::NodeWithType for CastExpr {}

impl com::github::javaparser::ast::node_types::node_with_expression::NodeWithExpression for CastExpr {}

impl /* Java */ java::lang::Cloneable /**/ for CastExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for CastExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for CastExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for CastExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for CastExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for CastExpr {}