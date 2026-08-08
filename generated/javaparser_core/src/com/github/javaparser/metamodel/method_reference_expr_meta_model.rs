use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::MethodReferenceExpr;
use java::util::Optional;

pub struct MethodReferenceExprMetaModel {
	identifier_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	scope_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	type_arguments_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	using_diamond_operator_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl MethodReferenceExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::method_reference_expr_meta_model::MethodReferenceExprMetaModel {
		super(super_base_node_meta_model, MethodReferenceExpr.class, "MethodReferenceExpr", "com.github.javaparser.ast.expr", false, false);
	}
}