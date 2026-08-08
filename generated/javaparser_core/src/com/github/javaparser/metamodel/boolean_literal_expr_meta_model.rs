use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::BooleanLiteralExpr;
use java::util::Optional;

pub struct BooleanLiteralExprMetaModel {
	value_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl BooleanLiteralExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::boolean_literal_expr_meta_model::BooleanLiteralExprMetaModel {
		super(super_base_node_meta_model, BooleanLiteralExpr.class, "BooleanLiteralExpr", "com.github.javaparser.ast.expr", false, false);
	}
}