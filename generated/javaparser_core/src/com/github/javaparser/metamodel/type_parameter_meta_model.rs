use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::type::TypeParameter;
use java::util::Optional;

pub struct TypeParameterMetaModel {
	name_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	type_bound_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl TypeParameterMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::type_parameter_meta_model::TypeParameterMetaModel {
		super(super_base_node_meta_model, TypeParameter.class, "TypeParameter", "com.github.javaparser.ast.type", false, false);
	}
}