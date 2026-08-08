use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::CastExpr;
use java::util::Optional;

pub struct CastExprMetaModel {
	expression_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	type_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl CastExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::cast_expr_meta_model::CastExprMetaModel {
		super(super_base_node_meta_model, CastExpr.class, "CastExpr", "com.github.javaparser.ast.expr", false, false);
	}
}