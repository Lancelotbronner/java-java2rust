use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::ConditionalExpr;
use java::util::Optional;

pub struct ConditionalExprMetaModel {
	condition_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	else_expr_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	then_expr_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ConditionalExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::conditional_expr_meta_model::ConditionalExprMetaModel {
		super(super_base_node_meta_model, ConditionalExpr.class, "ConditionalExpr", "com.github.javaparser.ast.expr", false, false);
	}
}