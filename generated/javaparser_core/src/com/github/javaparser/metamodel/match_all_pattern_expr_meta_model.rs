use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::MatchAllPatternExpr;
use java::util::Optional;

pub struct MatchAllPatternExprMetaModel {
	modifiers_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl MatchAllPatternExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::match_all_pattern_expr_meta_model::MatchAllPatternExprMetaModel {
		super(super_base_node_meta_model, MatchAllPatternExpr.class, "MatchAllPatternExpr", "com.github.javaparser.ast.expr", false, false);
	}
}