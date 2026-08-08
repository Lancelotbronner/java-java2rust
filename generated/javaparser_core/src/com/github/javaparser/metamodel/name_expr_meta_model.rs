use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::NameExpr;
use java::util::Optional;

pub struct NameExprMetaModel {
	name_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl NameExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::name_expr_meta_model::NameExprMetaModel {
		super(super_base_node_meta_model, NameExpr.class, "NameExpr", "com.github.javaparser.ast.expr", false, false);
	}
}