use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::RecordPatternExpr;
use java::util::Optional;

pub struct RecordPatternExprMetaModel {
	modifiers_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	pattern_list_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl RecordPatternExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::record_pattern_expr_meta_model::RecordPatternExprMetaModel {
		super(super_base_node_meta_model, RecordPatternExpr.class, "RecordPatternExpr", "com.github.javaparser.ast.expr", false, false);
	}
}