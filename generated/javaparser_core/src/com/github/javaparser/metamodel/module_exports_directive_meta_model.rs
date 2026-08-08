use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::modules::ModuleExportsDirective;
use java::util::Optional;

pub struct ModuleExportsDirectiveMetaModel {
	module_names_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	name_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ModuleExportsDirectiveMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::module_exports_directive_meta_model::ModuleExportsDirectiveMetaModel {
		super(super_base_node_meta_model, ModuleExportsDirective.class, "ModuleExportsDirective", "com.github.javaparser.ast.modules", false, false);
	}
}