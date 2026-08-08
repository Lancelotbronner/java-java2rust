use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::modules::ModuleOpensDirective;
use java::util::Optional;

pub struct ModuleOpensDirectiveMetaModel {
	module_names_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	name_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ModuleOpensDirectiveMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::module_opens_directive_meta_model::ModuleOpensDirectiveMetaModel {
		super(super_base_node_meta_model, ModuleOpensDirective.class, "ModuleOpensDirective", "com.github.javaparser.ast.modules", false, false);
	}
}