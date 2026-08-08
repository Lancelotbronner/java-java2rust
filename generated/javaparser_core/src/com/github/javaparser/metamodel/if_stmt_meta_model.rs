use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::stmt::IfStmt;
use java::util::Optional;

pub struct IfStmtMetaModel {
	condition_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	else_stmt_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	then_stmt_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	cascading_if_stmt_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	else_block_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	else_branch_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	then_block_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl IfStmtMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::if_stmt_meta_model::IfStmtMetaModel {
		super(super_base_node_meta_model, IfStmt.class, "IfStmt", "com.github.javaparser.ast.stmt", false, false);
	}
}