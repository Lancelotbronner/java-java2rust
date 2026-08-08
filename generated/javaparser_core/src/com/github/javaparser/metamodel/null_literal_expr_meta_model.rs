use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::NullLiteralExpr;
use java::util::Optional;

pub struct NullLiteralExprMetaModel;

impl NullLiteralExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::null_literal_expr_meta_model::NullLiteralExprMetaModel {
		super(super_base_node_meta_model, NullLiteralExpr.class, "NullLiteralExpr", "com.github.javaparser.ast.expr", false, false);
	}
}