use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::type::UnknownType;
use java::util::Optional;

pub struct UnknownTypeMetaModel;

impl UnknownTypeMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::unknown_type_meta_model::UnknownTypeMetaModel {
		super(super_base_node_meta_model, UnknownType.class, "UnknownType", "com.github.javaparser.ast.type", false, false);
	}
}