use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::LambdaExpr;
use java::util::Optional;

pub struct LambdaExprMetaModel {
	body_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	is_enclosing_parameters_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	parameters_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	expression_body_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl LambdaExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::lambda_expr_meta_model::LambdaExprMetaModel {
		super(super_base_node_meta_model, LambdaExpr.class, "LambdaExpr", "com.github.javaparser.ast.expr", false, false);
	}
}