use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::AssignExpr;
use java::util::Optional;

pub struct AssignExprMetaModel {
	operator_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	target_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	value_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl AssignExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::assign_expr_meta_model::AssignExprMetaModel {
		super(super_base_node_meta_model, AssignExpr.class, "AssignExpr", "com.github.javaparser.ast.expr", false, false);
	}
}