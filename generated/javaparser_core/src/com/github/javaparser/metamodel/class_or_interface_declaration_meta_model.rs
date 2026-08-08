use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::body::ClassOrInterfaceDeclaration;
use java::util::Optional;

pub struct ClassOrInterfaceDeclarationMetaModel {
	extended_types_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	implemented_types_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	is_compact_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	is_interface_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	permitted_types_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	type_parameters_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ClassOrInterfaceDeclarationMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::class_or_interface_declaration_meta_model::ClassOrInterfaceDeclarationMetaModel {
		super(super_base_node_meta_model, ClassOrInterfaceDeclaration.class, "ClassOrInterfaceDeclaration", "com.github.javaparser.ast.body", false, false);
	}
}