use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::BinaryExpr;
use java::util::Optional;

pub struct BinaryExprMetaModel {
	left_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	operator_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	right_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl BinaryExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::binary_expr_meta_model::BinaryExprMetaModel {
		super(super_base_node_meta_model, BinaryExpr.class, "BinaryExpr", "com.github.javaparser.ast.expr", false, false);
	}
}