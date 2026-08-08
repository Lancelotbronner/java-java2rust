use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::ArrayCreationExpr;
use java::util::Optional;

pub struct ArrayCreationExprMetaModel {
	element_type_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	initializer_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	levels_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ArrayCreationExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::array_creation_expr_meta_model::ArrayCreationExprMetaModel {
		super(super_base_node_meta_model, ArrayCreationExpr.class, "ArrayCreationExpr", "com.github.javaparser.ast.expr", false, false);
	}
}