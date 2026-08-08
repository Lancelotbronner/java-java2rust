use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::Name;
use java::util::Optional;

pub struct NameMetaModel {
	identifier_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	qualifier_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl NameMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::name_meta_model::NameMetaModel {
		super(super_base_node_meta_model, Name.class, "Name", "com.github.javaparser.ast.expr", false, false);
	}
}