use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::modules::ModuleDeclaration;
use java::util::Optional;

pub struct ModuleDeclarationMetaModel {
	annotations_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	directives_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	is_open_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	name_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ModuleDeclarationMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::module_declaration_meta_model::ModuleDeclarationMetaModel {
		super(super_base_node_meta_model, ModuleDeclaration.class, "ModuleDeclaration", "com.github.javaparser.ast.modules", false, false);
	}
}