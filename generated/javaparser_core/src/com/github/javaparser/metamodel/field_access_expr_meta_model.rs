use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::FieldAccessExpr;
use java::util::Optional;

pub struct FieldAccessExprMetaModel {
	name_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	scope_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	type_arguments_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	using_diamond_operator_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl FieldAccessExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::field_access_expr_meta_model::FieldAccessExprMetaModel {
		super(super_base_node_meta_model, FieldAccessExpr.class, "FieldAccessExpr", "com.github.javaparser.ast.expr", false, false);
	}
}