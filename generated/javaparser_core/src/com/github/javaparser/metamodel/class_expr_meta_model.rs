use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::ClassExpr;
use java::util::Optional;

pub struct ClassExprMetaModel {
	type_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ClassExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::class_expr_meta_model::ClassExprMetaModel {
		super(super_base_node_meta_model, ClassExpr.class, "ClassExpr", "com.github.javaparser.ast.expr", false, false);
	}
}