use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::SimpleName;
use java::util::Optional;

pub struct SimpleNameMetaModel {
	identifier_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl SimpleNameMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::simple_name_meta_model::SimpleNameMetaModel {
		super(super_base_node_meta_model, SimpleName.class, "SimpleName", "com.github.javaparser.ast.expr", false, false);
	}
}