use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::body::AnnotationDeclaration;
use java::util::Optional;

pub struct AnnotationDeclarationMetaModel;

impl AnnotationDeclarationMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::annotation_declaration_meta_model::AnnotationDeclarationMetaModel {
		super(super_base_node_meta_model, AnnotationDeclaration.class, "AnnotationDeclaration", "com.github.javaparser.ast.body", false, false);
	}
}