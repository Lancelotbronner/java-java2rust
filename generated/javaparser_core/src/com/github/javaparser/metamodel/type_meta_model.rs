use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::type::Type;
use java::util::Optional;

pub struct TypeMetaModel {
	annotations_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl TypeMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::type_meta_model::TypeMetaModel {
		super(super_base_node_meta_model, Type.class, "Type", "com.github.javaparser.ast.type", true, false);
	}

	fn new(super_node_meta_model: &/* Java */ java::util::Optional /**/, type: &/* Java */ java::lang::Class /**/, name: &/* Java */ java::lang::String /**/, package_name: &/* Java */ java::lang::String /**/, is_abstract: bool, has_wildcard: bool) -> com::github::javaparser::metamodel::type_meta_model::TypeMetaModel {
		super(super_node_meta_model, type, name, package_name, is_abstract, has_wildcard);
	}
}