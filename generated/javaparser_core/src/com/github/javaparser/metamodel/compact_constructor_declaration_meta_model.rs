use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::body::CompactConstructorDeclaration;
use java::util::Optional;

pub struct CompactConstructorDeclarationMetaModel {
	body_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	modifiers_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	name_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	thrown_exceptions_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	type_parameters_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl CompactConstructorDeclarationMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::compact_constructor_declaration_meta_model::CompactConstructorDeclarationMetaModel {
		super(super_base_node_meta_model, CompactConstructorDeclaration.class, "CompactConstructorDeclaration", "com.github.javaparser.ast.body", false, false);
	}
}