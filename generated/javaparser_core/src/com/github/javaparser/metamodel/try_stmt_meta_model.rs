use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::stmt::TryStmt;
use java::util::Optional;

pub struct TryStmtMetaModel {
	catch_clauses_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	finally_block_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	resources_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	try_block_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl TryStmtMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::try_stmt_meta_model::TryStmtMetaModel {
		super(super_base_node_meta_model, TryStmt.class, "TryStmt", "com.github.javaparser.ast.stmt", false, false);
	}
}