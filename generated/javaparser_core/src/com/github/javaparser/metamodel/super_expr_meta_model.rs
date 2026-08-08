use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::SuperExpr;
use java::util::Optional;

pub struct SuperExprMetaModel {
	type_name_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl SuperExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::super_expr_meta_model::SuperExprMetaModel {
		super(super_base_node_meta_model, SuperExpr.class, "SuperExpr", "com.github.javaparser.ast.expr", false, false);
	}
}