use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::LongLiteralExpr;
use java::util::Optional;

pub struct LongLiteralExprMetaModel;

impl LongLiteralExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::long_literal_expr_meta_model::LongLiteralExprMetaModel {
		super(super_base_node_meta_model, LongLiteralExpr.class, "LongLiteralExpr", "com.github.javaparser.ast.expr", false, false);
	}
}