use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::body::TypeDeclaration;
use java::util::Optional;

pub struct TypeDeclarationMetaModel {
	members_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	modifiers_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	name_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl TypeDeclarationMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::type_declaration_meta_model::TypeDeclarationMetaModel {
		super(super_base_node_meta_model, TypeDeclaration.class, "TypeDeclaration", "com.github.javaparser.ast.body", true, true);
	}

	fn new(super_node_meta_model: &/* Java */ java::util::Optional /**/, type: &/* Java */ java::lang::Class /**/, name: &/* Java */ java::lang::String /**/, package_name: &/* Java */ java::lang::String /**/, is_abstract: bool, has_wildcard: bool) -> com::github::javaparser::metamodel::type_declaration_meta_model::TypeDeclarationMetaModel {
		super(super_node_meta_model, type, name, package_name, is_abstract, has_wildcard);
	}
}