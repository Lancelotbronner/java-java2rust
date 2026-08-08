use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::NormalAnnotationExpr;
use java::util::Optional;

pub struct NormalAnnotationExprMetaModel {
	pairs_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl NormalAnnotationExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::normal_annotation_expr_meta_model::NormalAnnotationExprMetaModel {
		super(super_base_node_meta_model, NormalAnnotationExpr.class, "NormalAnnotationExpr", "com.github.javaparser.ast.expr", false, false);
	}
}