use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::ArrayInitializerExpr;
use java::util::Optional;

pub struct ArrayInitializerExprMetaModel {
	values_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ArrayInitializerExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::array_initializer_expr_meta_model::ArrayInitializerExprMetaModel {
		super(super_base_node_meta_model, ArrayInitializerExpr.class, "ArrayInitializerExpr", "com.github.javaparser.ast.expr", false, false);
	}
}