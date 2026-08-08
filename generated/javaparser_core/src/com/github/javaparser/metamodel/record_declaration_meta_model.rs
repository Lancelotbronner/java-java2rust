use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::body::RecordDeclaration;
use java::util::Optional;

pub struct RecordDeclarationMetaModel {
	implemented_types_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	parameters_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	receiver_parameter_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	type_parameters_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl RecordDeclarationMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::record_declaration_meta_model::RecordDeclarationMetaModel {
		super(super_base_node_meta_model, RecordDeclaration.class, "RecordDeclaration", "com.github.javaparser.ast.body", false, false);
	}
}