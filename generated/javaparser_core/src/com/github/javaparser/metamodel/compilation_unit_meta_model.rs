use crate::com::github::javaparser::ast::CompilationUnit;
use crate::com::github::javaparser::ast::Generated;
use java::util::Optional;

pub struct CompilationUnitMetaModel {
	imports_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	module_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	package_declaration_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	types_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl CompilationUnitMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::compilation_unit_meta_model::CompilationUnitMetaModel {
		super(super_base_node_meta_model, CompilationUnit.class, "CompilationUnit", "com.github.javaparser.ast", false, false);
	}
}