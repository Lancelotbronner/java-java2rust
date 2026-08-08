use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::type::WildcardType;
use java::util::Optional;

pub struct WildcardTypeMetaModel {
	extended_type_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	super_type_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl WildcardTypeMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::wildcard_type_meta_model::WildcardTypeMetaModel {
		super(super_base_node_meta_model, WildcardType.class, "WildcardType", "com.github.javaparser.ast.type", false, false);
	}
}