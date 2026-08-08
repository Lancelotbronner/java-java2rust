use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::ImportDeclaration;
use java::util::Optional;

pub struct ImportDeclarationMetaModel {
	is_asterisk_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	is_module_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	is_static_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	name_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ImportDeclarationMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::import_declaration_meta_model::ImportDeclarationMetaModel {
		super(super_base_node_meta_model, ImportDeclaration.class, "ImportDeclaration", "com.github.javaparser.ast", false, false);
	}
}