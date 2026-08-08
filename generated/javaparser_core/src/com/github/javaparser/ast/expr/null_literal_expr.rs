use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::NullLiteralExprMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct NullLiteralExpr;

impl NullLiteralExpr {
	pub fn new() -> com::github::javaparser::ast::expr::null_literal_expr::NullLiteralExpr {
		this(null);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange) -> com::github::javaparser::ast::expr::null_literal_expr::NullLiteralExpr {
		super(token_range);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::null_literal_expr::NullLiteralExpr {
		return self.accept(CloneVisitor::new(), null) as NullLiteralExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::null_literal_expr_meta_model::NullLiteralExprMetaModel {
		return JavaParserMetaModel::nullLiteralExprMetaModel;
	}

	pub fn is_null_literal_expr(&self) -> bool {
		return true;
	}

	pub fn as_null_literal_expr(&self) -> com::github::javaparser::ast::expr::null_literal_expr::NullLiteralExpr {
		return self;
	}

	pub fn if_null_literal_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_null_literal_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for NullLiteralExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for NullLiteralExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for NullLiteralExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for NullLiteralExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for NullLiteralExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for NullLiteralExpr {}