use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::ArrayAccessExpr;
use java::util::Optional;

pub struct ArrayAccessExprMetaModel {
	index_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	name_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ArrayAccessExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::array_access_expr_meta_model::ArrayAccessExprMetaModel {
		super(super_base_node_meta_model, ArrayAccessExpr.class, "ArrayAccessExpr", "com.github.javaparser.ast.expr", false, false);
	}
}