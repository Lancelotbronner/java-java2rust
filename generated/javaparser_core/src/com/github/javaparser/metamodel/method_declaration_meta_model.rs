use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::body::MethodDeclaration;
use java::util::Optional;

pub struct MethodDeclarationMetaModel {
	body_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	type_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl MethodDeclarationMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::method_declaration_meta_model::MethodDeclarationMetaModel {
		super(super_base_node_meta_model, MethodDeclaration.class, "MethodDeclaration", "com.github.javaparser.ast.body", false, false);
	}
}