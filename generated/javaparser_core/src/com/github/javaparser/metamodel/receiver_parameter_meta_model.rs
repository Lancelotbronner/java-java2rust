use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::body::ReceiverParameter;
use java::util::Optional;

pub struct ReceiverParameterMetaModel {
	annotations_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	name_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	type_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ReceiverParameterMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::receiver_parameter_meta_model::ReceiverParameterMetaModel {
		super(super_base_node_meta_model, ReceiverParameter.class, "ReceiverParameter", "com.github.javaparser.ast.body", false, false);
	}
}