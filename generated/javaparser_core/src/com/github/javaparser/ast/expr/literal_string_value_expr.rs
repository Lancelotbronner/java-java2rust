use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::LiteralStringValueExprMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct LiteralStringValueExpr {
	value: /* Java */ java::lang::String /**/,
}

impl LiteralStringValueExpr {
	pub fn new(value: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::literal_string_value_expr::LiteralStringValueExpr {
		this(null, value);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, value: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::literal_string_value_expr::LiteralStringValueExpr {
		super(token_range);
		self.set_value(value);
		self.custom_initialization();
	}

	pub fn get_value(&self) -> /* Java */ java::lang::String /**/ {
		return self.value;
	}

	pub fn set_value(&mut self, value: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::literal_string_value_expr::LiteralStringValueExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(value)?;
		if value.equals(self.value) {
			return self;
		}
		self.notify_property_change(ObservableProperty::VALUE, self.value, value);
		self.value = value;
		return self;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::literal_string_value_expr::LiteralStringValueExpr {
		return self.accept(CloneVisitor::new(), null) as LiteralStringValueExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::literal_string_value_expr_meta_model::LiteralStringValueExprMetaModel {
		return JavaParserMetaModel::literalStringValueExprMetaModel;
	}

	pub fn is_literal_string_value_expr(&self) -> bool {
		return true;
	}

	pub fn as_literal_string_value_expr(&self) -> com::github::javaparser::ast::expr::literal_string_value_expr::LiteralStringValueExpr {
		return self;
	}

	pub fn if_literal_string_value_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_literal_string_value_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for LiteralStringValueExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for LiteralStringValueExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for LiteralStringValueExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for LiteralStringValueExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for LiteralStringValueExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for LiteralStringValueExpr {}