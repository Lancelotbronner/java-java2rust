use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::VariableDeclarationExpr;
use java::util::Optional;

pub struct VariableDeclarationExprMetaModel {
	annotations_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	modifiers_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	variables_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	maximum_common_type_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl VariableDeclarationExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::variable_declaration_expr_meta_model::VariableDeclarationExprMetaModel {
		super(super_base_node_meta_model, VariableDeclarationExpr.class, "VariableDeclarationExpr", "com.github.javaparser.ast.expr", false, false);
	}
}