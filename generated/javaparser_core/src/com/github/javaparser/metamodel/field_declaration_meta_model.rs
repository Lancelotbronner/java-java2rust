use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::body::FieldDeclaration;
use java::util::Optional;

pub struct FieldDeclarationMetaModel {
	modifiers_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	variables_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	maximum_common_type_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl FieldDeclarationMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::field_declaration_meta_model::FieldDeclarationMetaModel {
		super(super_base_node_meta_model, FieldDeclaration.class, "FieldDeclaration", "com.github.javaparser.ast.body", false, false);
	}
}