use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::BooleanLiteralExprMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct BooleanLiteralExpr {
	value: bool,
}

impl BooleanLiteralExpr {
	pub fn new() -> com::github::javaparser::ast::expr::boolean_literal_expr::BooleanLiteralExpr {
		this(null, false);
	}

	pub fn new(value: bool) -> com::github::javaparser::ast::expr::boolean_literal_expr::BooleanLiteralExpr {
		this(null, value);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, value: bool) -> com::github::javaparser::ast::expr::boolean_literal_expr::BooleanLiteralExpr {
		super(token_range);
		self.set_value(value);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn is_value(&self) -> bool {
		return self.value;
	}

	pub fn get_value(&self) -> bool {
		return self.is_value();
	}

	pub fn set_value(&mut self, value: bool) -> com::github::javaparser::ast::expr::boolean_literal_expr::BooleanLiteralExpr {
		if value == self.value {
			return self;
		}
		self.notify_property_change(ObservableProperty::VALUE, self.value, value);
		self.value = value;
		return self;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::boolean_literal_expr::BooleanLiteralExpr {
		return self.accept(CloneVisitor::new(), null) as BooleanLiteralExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::boolean_literal_expr_meta_model::BooleanLiteralExprMetaModel {
		return JavaParserMetaModel::booleanLiteralExprMetaModel;
	}

	pub fn is_boolean_literal_expr(&self) -> bool {
		return true;
	}

	pub fn as_boolean_literal_expr(&self) -> com::github::javaparser::ast::expr::boolean_literal_expr::BooleanLiteralExpr {
		return self;
	}

	pub fn if_boolean_literal_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_boolean_literal_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for BooleanLiteralExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for BooleanLiteralExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for BooleanLiteralExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for BooleanLiteralExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for BooleanLiteralExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for BooleanLiteralExpr {}