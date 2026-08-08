use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::type::ArrayType;
use java::util::Optional;

pub struct ArrayTypeMetaModel {
	component_type_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	origin_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ArrayTypeMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::array_type_meta_model::ArrayTypeMetaModel {
		super(super_base_node_meta_model, ArrayType.class, "ArrayType", "com.github.javaparser.ast.type", false, false);
	}
}