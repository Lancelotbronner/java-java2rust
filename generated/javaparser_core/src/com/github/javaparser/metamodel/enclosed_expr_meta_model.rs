use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::EnclosedExpr;
use java::util::Optional;

pub struct EnclosedExprMetaModel {
	inner_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl EnclosedExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::enclosed_expr_meta_model::EnclosedExprMetaModel {
		super(super_base_node_meta_model, EnclosedExpr.class, "EnclosedExpr", "com.github.javaparser.ast.expr", false, false);
	}
}