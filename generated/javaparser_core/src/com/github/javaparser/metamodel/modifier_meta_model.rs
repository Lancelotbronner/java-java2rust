use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Modifier;
use java::util::Optional;

pub struct ModifierMetaModel {
	keyword_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ModifierMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::modifier_meta_model::ModifierMetaModel {
		super(super_base_node_meta_model, Modifier.class, "Modifier", "com.github.javaparser.ast", false, false);
	}
}