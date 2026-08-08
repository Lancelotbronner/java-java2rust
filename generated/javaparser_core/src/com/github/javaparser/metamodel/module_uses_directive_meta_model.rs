use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::modules::ModuleUsesDirective;
use java::util::Optional;

pub struct ModuleUsesDirectiveMetaModel {
	name_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ModuleUsesDirectiveMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::module_uses_directive_meta_model::ModuleUsesDirectiveMetaModel {
		super(super_base_node_meta_model, ModuleUsesDirective.class, "ModuleUsesDirective", "com.github.javaparser.ast.modules", false, false);
	}
}