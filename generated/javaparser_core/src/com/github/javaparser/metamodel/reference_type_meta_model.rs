use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::type::ReferenceType;
use java::util::Optional;

pub struct ReferenceTypeMetaModel;

impl ReferenceTypeMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::reference_type_meta_model::ReferenceTypeMetaModel {
		super(super_base_node_meta_model, ReferenceType.class, "ReferenceType", "com.github.javaparser.ast.type", true, false);
	}

	fn new(super_node_meta_model: &/* Java */ java::util::Optional /**/, type: &/* Java */ java::lang::Class /**/, name: &/* Java */ java::lang::String /**/, package_name: &/* Java */ java::lang::String /**/, is_abstract: bool, has_wildcard: bool) -> com::github::javaparser::metamodel::reference_type_meta_model::ReferenceTypeMetaModel {
		super(super_node_meta_model, type, name, package_name, is_abstract, has_wildcard);
	}
}