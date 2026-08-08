use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::body::CallableDeclaration;
use java::util::Optional;

pub struct CallableDeclarationMetaModel {
	modifiers_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	name_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	parameters_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	receiver_parameter_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	thrown_exceptions_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	type_parameters_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl CallableDeclarationMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::callable_declaration_meta_model::CallableDeclarationMetaModel {
		super(super_base_node_meta_model, CallableDeclaration.class, "CallableDeclaration", "com.github.javaparser.ast.body", true, true);
	}

	fn new(super_node_meta_model: &/* Java */ java::util::Optional /**/, type: &/* Java */ java::lang::Class /**/, name: &/* Java */ java::lang::String /**/, package_name: &/* Java */ java::lang::String /**/, is_abstract: bool, has_wildcard: bool) -> com::github::javaparser::metamodel::callable_declaration_meta_model::CallableDeclarationMetaModel {
		super(super_node_meta_model, type, name, package_name, is_abstract, has_wildcard);
	}
}