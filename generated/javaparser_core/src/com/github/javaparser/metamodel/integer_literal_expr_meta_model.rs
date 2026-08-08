use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::IntegerLiteralExpr;
use java::util::Optional;

pub struct IntegerLiteralExprMetaModel;

impl IntegerLiteralExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::integer_literal_expr_meta_model::IntegerLiteralExprMetaModel {
		super(super_base_node_meta_model, IntegerLiteralExpr.class, "IntegerLiteralExpr", "com.github.javaparser.ast.expr", false, false);
	}
}