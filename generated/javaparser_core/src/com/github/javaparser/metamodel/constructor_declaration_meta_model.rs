use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::body::ConstructorDeclaration;
use java::util::Optional;

pub struct ConstructorDeclarationMetaModel {
	body_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ConstructorDeclarationMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::constructor_declaration_meta_model::ConstructorDeclarationMetaModel {
		super(super_base_node_meta_model, ConstructorDeclaration.class, "ConstructorDeclaration", "com.github.javaparser.ast.body", false, false);
	}
}