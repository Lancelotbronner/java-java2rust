use crate::com::github::javaparser::ast::ArrayCreationLevel;
use crate::com::github::javaparser::ast::Generated;
use java::util::Optional;

pub struct ArrayCreationLevelMetaModel {
	annotations_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	dimension_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ArrayCreationLevelMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::array_creation_level_meta_model::ArrayCreationLevelMetaModel {
		super(super_base_node_meta_model, ArrayCreationLevel.class, "ArrayCreationLevel", "com.github.javaparser.ast", false, false);
	}
}