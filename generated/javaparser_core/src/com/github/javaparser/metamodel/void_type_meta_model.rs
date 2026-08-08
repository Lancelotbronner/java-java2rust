use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::type::VoidType;
use java::util::Optional;

pub struct VoidTypeMetaModel;

impl VoidTypeMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::void_type_meta_model::VoidTypeMetaModel {
		super(super_base_node_meta_model, VoidType.class, "VoidType", "com.github.javaparser.ast.type", false, false);
	}
}