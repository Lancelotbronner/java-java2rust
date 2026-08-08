use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::ThisExpr;
use java::util::Optional;

pub struct ThisExprMetaModel {
	type_name_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ThisExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::this_expr_meta_model::ThisExprMetaModel {
		super(super_base_node_meta_model, ThisExpr.class, "ThisExpr", "com.github.javaparser.ast.expr", false, false);
	}
}