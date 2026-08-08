use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::type::ClassOrInterfaceType;
use java::util::Optional;

pub struct ClassOrInterfaceTypeMetaModel {
	name_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	scope_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	type_arguments_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	using_diamond_operator_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ClassOrInterfaceTypeMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::class_or_interface_type_meta_model::ClassOrInterfaceTypeMetaModel {
		super(super_base_node_meta_model, ClassOrInterfaceType.class, "ClassOrInterfaceType", "com.github.javaparser.ast.type", false, false);
	}
}