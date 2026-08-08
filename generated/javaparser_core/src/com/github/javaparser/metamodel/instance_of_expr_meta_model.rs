use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::InstanceOfExpr;
use java::util::Optional;

pub struct InstanceOfExprMetaModel {
	expression_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	pattern_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	type_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl InstanceOfExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::instance_of_expr_meta_model::InstanceOfExprMetaModel {
		super(super_base_node_meta_model, InstanceOfExpr.class, "InstanceOfExpr", "com.github.javaparser.ast.expr", false, false);
	}
}