use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::expr::PatternExpr;
use java::util::Optional;

pub struct PatternExprMetaModel {
	type_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl PatternExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::pattern_expr_meta_model::PatternExprMetaModel {
		super(super_base_node_meta_model, PatternExpr.class, "PatternExpr", "com.github.javaparser.ast.expr", true, false);
	}

	fn new(super_node_meta_model: &/* Java */ java::util::Optional /**/, type: &/* Java */ java::lang::Class /**/, name: &/* Java */ java::lang::String /**/, package_name: &/* Java */ java::lang::String /**/, is_abstract: bool, has_wildcard: bool) -> com::github::javaparser::metamodel::pattern_expr_meta_model::PatternExprMetaModel {
		super(super_node_meta_model, type, name, package_name, is_abstract, has_wildcard);
	}
}