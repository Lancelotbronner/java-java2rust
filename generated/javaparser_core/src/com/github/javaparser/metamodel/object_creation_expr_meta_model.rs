use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::ObjectCreationExpr;
use java::util::Optional;

pub struct ObjectCreationExprMetaModel {
	anonymous_class_body_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	arguments_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	scope_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	type_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	type_arguments_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	using_diamond_operator_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ObjectCreationExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::object_creation_expr_meta_model::ObjectCreationExprMetaModel {
		super(super_base_node_meta_model, ObjectCreationExpr.class, "ObjectCreationExpr", "com.github.javaparser.ast.expr", false, false);
	}
}