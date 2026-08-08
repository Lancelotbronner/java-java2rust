use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::type::VarType;
use java::util::Optional;

pub struct VarTypeMetaModel;

impl VarTypeMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::var_type_meta_model::VarTypeMetaModel {
		super(super_base_node_meta_model, VarType.class, "VarType", "com.github.javaparser.ast.type", false, false);
	}
}