use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::UnaryExpr;
use java::util::Optional;

pub struct UnaryExprMetaModel {
	expression_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	operator_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	postfix_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	prefix_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl UnaryExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::unary_expr_meta_model::UnaryExprMetaModel {
		super(super_base_node_meta_model, UnaryExpr.class, "UnaryExpr", "com.github.javaparser.ast.expr", false, false);
	}
}