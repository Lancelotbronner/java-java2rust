use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::body::Parameter;
use java::util::Optional;

pub struct ParameterMetaModel {
	annotations_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	is_var_args_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	modifiers_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	name_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	type_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	var_args_annotations_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ParameterMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::parameter_meta_model::ParameterMetaModel {
		super(super_base_node_meta_model, Parameter.class, "Parameter", "com.github.javaparser.ast.body", false, false);
	}
}