use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::DoubleLiteralExpr;
use java::util::Optional;

pub struct DoubleLiteralExprMetaModel;

impl DoubleLiteralExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::double_literal_expr_meta_model::DoubleLiteralExprMetaModel {
		super(super_base_node_meta_model, DoubleLiteralExpr.class, "DoubleLiteralExpr", "com.github.javaparser.ast.expr", false, false);
	}
}