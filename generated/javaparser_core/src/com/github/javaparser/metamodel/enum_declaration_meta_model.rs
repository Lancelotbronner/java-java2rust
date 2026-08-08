use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::body::EnumDeclaration;
use java::util::Optional;

pub struct EnumDeclarationMetaModel {
	entries_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	implemented_types_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl EnumDeclarationMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::enum_declaration_meta_model::EnumDeclarationMetaModel {
		super(super_base_node_meta_model, EnumDeclaration.class, "EnumDeclaration", "com.github.javaparser.ast.body", false, false);
	}
}