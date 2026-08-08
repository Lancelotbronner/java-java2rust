use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::body::VariableDeclarator;
use java::util::Optional;

pub struct VariableDeclaratorMetaModel {
	initializer_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	name_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	type_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl VariableDeclaratorMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::variable_declarator_meta_model::VariableDeclaratorMetaModel {
		super(super_base_node_meta_model, VariableDeclarator.class, "VariableDeclarator", "com.github.javaparser.ast.body", false, false);
	}
}