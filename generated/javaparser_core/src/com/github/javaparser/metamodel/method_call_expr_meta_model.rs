use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::MethodCallExpr;
use java::util::Optional;

pub struct MethodCallExprMetaModel {
	arguments_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	name_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	scope_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	type_arguments_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	using_diamond_operator_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl MethodCallExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::method_call_expr_meta_model::MethodCallExprMetaModel {
		super(super_base_node_meta_model, MethodCallExpr.class, "MethodCallExpr", "com.github.javaparser.ast.expr", false, false);
	}
}