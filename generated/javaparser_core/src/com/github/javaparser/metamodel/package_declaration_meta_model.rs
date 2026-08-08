use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::PackageDeclaration;
use java::util::Optional;

pub struct PackageDeclarationMetaModel {
	annotations_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	name_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl PackageDeclarationMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::package_declaration_meta_model::PackageDeclarationMetaModel {
		super(super_base_node_meta_model, PackageDeclaration.class, "PackageDeclaration", "com.github.javaparser.ast", false, false);
	}
}