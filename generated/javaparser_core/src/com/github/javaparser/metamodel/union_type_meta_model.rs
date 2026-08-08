use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::type::UnionType;
use java::util::Optional;

pub struct UnionTypeMetaModel {
	elements_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl UnionTypeMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::union_type_meta_model::UnionTypeMetaModel {
		super(super_base_node_meta_model, UnionType.class, "UnionType", "com.github.javaparser.ast.type", false, false);
	}
}