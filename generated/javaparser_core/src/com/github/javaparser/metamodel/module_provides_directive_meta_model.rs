use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::modules::ModuleProvidesDirective;
use java::util::Optional;

pub struct ModuleProvidesDirectiveMetaModel {
	name_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	with_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ModuleProvidesDirectiveMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::module_provides_directive_meta_model::ModuleProvidesDirectiveMetaModel {
		super(super_base_node_meta_model, ModuleProvidesDirective.class, "ModuleProvidesDirective", "com.github.javaparser.ast.modules", false, false);
	}
}