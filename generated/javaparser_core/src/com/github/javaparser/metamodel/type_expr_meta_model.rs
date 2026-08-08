use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::TypeExpr;
use java::util::Optional;

pub struct TypeExprMetaModel {
	type_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl TypeExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::type_expr_meta_model::TypeExprMetaModel {
		super(super_base_node_meta_model, TypeExpr.class, "TypeExpr", "com.github.javaparser.ast.expr", false, false);
	}
}