use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::type::IntersectionType;
use java::util::Optional;

pub struct IntersectionTypeMetaModel {
	elements_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl IntersectionTypeMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::intersection_type_meta_model::IntersectionTypeMetaModel {
		super(super_base_node_meta_model, IntersectionType.class, "IntersectionType", "com.github.javaparser.ast.type", false, false);
	}
}