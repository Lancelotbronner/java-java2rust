use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::stmt::ForEachStmt;
use java::util::Optional;

pub struct ForEachStmtMetaModel {
	body_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	iterable_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	variable_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ForEachStmtMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::for_each_stmt_meta_model::ForEachStmtMetaModel {
		super(super_base_node_meta_model, ForEachStmt.class, "ForEachStmt", "com.github.javaparser.ast.stmt", false, false);
	}
}