use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::stmt::ForStmt;
use java::util::Optional;

pub struct ForStmtMetaModel {
	body_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	compare_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	initialization_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	update_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ForStmtMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::for_stmt_meta_model::ForStmtMetaModel {
		super(super_base_node_meta_model, ForStmt.class, "ForStmt", "com.github.javaparser.ast.stmt", false, false);
	}
}