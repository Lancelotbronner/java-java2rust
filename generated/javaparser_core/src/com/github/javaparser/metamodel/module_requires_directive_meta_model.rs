use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::modules::ModuleRequiresDirective;
use java::util::Optional;

pub struct ModuleRequiresDirectiveMetaModel {
	modifiers_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	name_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ModuleRequiresDirectiveMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::module_requires_directive_meta_model::ModuleRequiresDirectiveMetaModel {
		super(super_base_node_meta_model, ModuleRequiresDirective.class, "ModuleRequiresDirective", "com.github.javaparser.ast.modules", false, false);
	}
}