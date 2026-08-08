use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::stmt::SwitchEntry;
use java::util::Optional;

pub struct SwitchEntryMetaModel {
	guard_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	is_default_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	labels_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	statements_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	type_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	switch_statement_entry_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl SwitchEntryMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::switch_entry_meta_model::SwitchEntryMetaModel {
		super(super_base_node_meta_model, SwitchEntry.class, "SwitchEntry", "com.github.javaparser.ast.stmt", false, false);
	}
}