use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::LiteralExprMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct LiteralExpr;

impl LiteralExpr {
	pub fn new() -> com::github::javaparser::ast::expr::literal_expr::LiteralExpr {
		this(null);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange) -> com::github::javaparser::ast::expr::literal_expr::LiteralExpr {
		super(token_range);
		self.custom_initialization();
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::literal_expr::LiteralExpr {
		return self.accept(CloneVisitor::new(), null) as LiteralExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::literal_expr_meta_model::LiteralExprMetaModel {
		return JavaParserMetaModel::literalExprMetaModel;
	}

	pub fn is_literal_expr(&self) -> bool {
		return true;
	}

	pub fn as_literal_expr(&self) -> com::github::javaparser::ast::expr::literal_expr::LiteralExpr {
		return self;
	}

	pub fn if_literal_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_literal_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for LiteralExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for LiteralExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for LiteralExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for LiteralExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for LiteralExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for LiteralExpr {}