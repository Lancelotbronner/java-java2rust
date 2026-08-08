use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::modules::ModuleDirective;
use java::util::Optional;

pub struct ModuleDirectiveMetaModel;

impl ModuleDirectiveMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::module_directive_meta_model::ModuleDirectiveMetaModel {
		super(super_base_node_meta_model, ModuleDirective.class, "ModuleDirective", "com.github.javaparser.ast.modules", true, false);
	}

	fn new(super_node_meta_model: &/* Java */ java::util::Optional /**/, type: &/* Java */ java::lang::Class /**/, name: &/* Java */ java::lang::String /**/, package_name: &/* Java */ java::lang::String /**/, is_abstract: bool, has_wildcard: bool) -> com::github::javaparser::metamodel::module_directive_meta_model::ModuleDirectiveMetaModel {
		super(super_node_meta_model, type, name, package_name, is_abstract, has_wildcard);
	}
}