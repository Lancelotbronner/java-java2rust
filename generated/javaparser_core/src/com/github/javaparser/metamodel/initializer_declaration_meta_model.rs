use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::body::InitializerDeclaration;
use java::util::Optional;

pub struct InitializerDeclarationMetaModel {
	body_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	is_static_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl InitializerDeclarationMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::initializer_declaration_meta_model::InitializerDeclarationMetaModel {
		super(super_base_node_meta_model, InitializerDeclaration.class, "InitializerDeclaration", "com.github.javaparser.ast.body", false, false);
	}
}