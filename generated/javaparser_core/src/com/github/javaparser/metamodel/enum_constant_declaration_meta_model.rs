use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::body::EnumConstantDeclaration;
use java::util::Optional;

pub struct EnumConstantDeclarationMetaModel {
	arguments_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	class_body_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	name_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl EnumConstantDeclarationMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::enum_constant_declaration_meta_model::EnumConstantDeclarationMetaModel {
		super(super_base_node_meta_model, EnumConstantDeclaration.class, "EnumConstantDeclaration", "com.github.javaparser.ast.body", false, false);
	}
}