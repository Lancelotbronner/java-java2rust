use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::SwitchExpr;
use java::util::Optional;

pub struct SwitchExprMetaModel {
	entries_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	selector_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl SwitchExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::switch_expr_meta_model::SwitchExprMetaModel {
		super(super_base_node_meta_model, SwitchExpr.class, "SwitchExpr", "com.github.javaparser.ast.expr", false, false);
	}
}