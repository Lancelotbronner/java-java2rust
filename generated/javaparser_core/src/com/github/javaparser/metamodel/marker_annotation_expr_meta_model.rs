use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::MarkerAnnotationExpr;
use java::util::Optional;

pub struct MarkerAnnotationExprMetaModel;

impl MarkerAnnotationExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::marker_annotation_expr_meta_model::MarkerAnnotationExprMetaModel {
		super(super_base_node_meta_model, MarkerAnnotationExpr.class, "MarkerAnnotationExpr", "com.github.javaparser.ast.expr", false, false);
	}
}