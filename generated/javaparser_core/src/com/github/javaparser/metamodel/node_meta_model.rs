use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use java::util::Optional;

pub struct NodeMetaModel {
	comment_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl NodeMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::node_meta_model::NodeMetaModel {
		super(super_base_node_meta_model, Node.class, "Node", "com.github.javaparser.ast", true, false);
	}

	fn new(super_node_meta_model: &/* Java */ java::util::Optional /**/, type: &/* Java */ java::lang::Class /**/, name: &/* Java */ java::lang::String /**/, package_name: &/* Java */ java::lang::String /**/, is_abstract: bool, has_wildcard: bool) -> com::github::javaparser::metamodel::node_meta_model::NodeMetaModel {
		super(super_node_meta_model, type, name, package_name, is_abstract, has_wildcard);
	}
}