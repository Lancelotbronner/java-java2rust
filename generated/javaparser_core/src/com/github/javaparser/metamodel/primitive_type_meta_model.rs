use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::type::PrimitiveType;
use java::util::Optional;

pub struct PrimitiveTypeMetaModel {
	type_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl PrimitiveTypeMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::primitive_type_meta_model::PrimitiveTypeMetaModel {
		super(super_base_node_meta_model, PrimitiveType.class, "PrimitiveType", "com.github.javaparser.ast.type", false, false);
	}
}