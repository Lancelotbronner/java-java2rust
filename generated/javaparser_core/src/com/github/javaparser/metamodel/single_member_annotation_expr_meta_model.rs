use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::SingleMemberAnnotationExpr;
use java::util::Optional;

pub struct SingleMemberAnnotationExprMetaModel {
	member_value_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl SingleMemberAnnotationExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::single_member_annotation_expr_meta_model::SingleMemberAnnotationExprMetaModel {
		super(super_base_node_meta_model, SingleMemberAnnotationExpr.class, "SingleMemberAnnotationExpr", "com.github.javaparser.ast.expr", false, false);
	}
}