use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::EnclosedExprMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct EnclosedExpr {
	inner: com::github::javaparser::ast::expr::expression::Expression,
}

impl EnclosedExpr {
	pub fn new() -> com::github::javaparser::ast::expr::enclosed_expr::EnclosedExpr {
		this(null, StringLiteralExpr::new());
	}

	pub fn new(inner: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::expr::enclosed_expr::EnclosedExpr {
		this(null, inner);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, inner: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::expr::enclosed_expr::EnclosedExpr {
		super(token_range);
		self.set_inner(inner);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_inner(&self) -> com::github::javaparser::ast::expr::expression::Expression {
		return self.inner;
	}

	pub fn set_inner(&mut self, inner: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::enclosed_expr::EnclosedExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(inner)?;
		if inner == self.inner {
			return self;
		}
		self.notify_property_change(ObservableProperty::INNER, self.inner, inner);
		if self.inner != null {
			self.inner.set_parent_node(null);
		}
	
		self.inner = inner;
		self.set_as_parent_node_of(inner);
		return self;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::enclosed_expr::EnclosedExpr {
		return self.accept(CloneVisitor::new(), null) as EnclosedExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::enclosed_expr_meta_model::EnclosedExprMetaModel {
		return JavaParserMetaModel::enclosedExprMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.inner {
			self.set_inner(replacement_node as Expression)?;
			return true;
		}
		return super.replace(node, replacement_node);
	}

	pub fn is_enclosed_expr(&self) -> bool {
		return true;
	}

	pub fn as_enclosed_expr(&self) -> com::github::javaparser::ast::expr::enclosed_expr::EnclosedExpr {
		return self;
	}

	pub fn if_enclosed_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_enclosed_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn is_poly_expression(&self) -> bool {
		return self.get_inner().is_poly_expression();
	}
}

impl /* Java */ java::lang::Cloneable /**/ for EnclosedExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for EnclosedExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for EnclosedExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for EnclosedExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for EnclosedExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for EnclosedExpr {}