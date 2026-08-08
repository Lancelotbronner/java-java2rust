use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::TypePatternExpr;
use java::util::Optional;

pub struct TypePatternExprMetaModel {
	modifiers_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	name_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl TypePatternExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::type_pattern_expr_meta_model::TypePatternExprMetaModel {
		super(super_base_node_meta_model, TypePatternExpr.class, "TypePatternExpr", "com.github.javaparser.ast.expr", false, false);
	}
}