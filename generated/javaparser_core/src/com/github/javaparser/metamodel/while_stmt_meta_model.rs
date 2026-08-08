use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::stmt::WhileStmt;
use java::util::Optional;

pub struct WhileStmtMetaModel {
	body_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	condition_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl WhileStmtMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::while_stmt_meta_model::WhileStmtMetaModel {
		super(super_base_node_meta_model, WhileStmt.class, "WhileStmt", "com.github.javaparser.ast.stmt", false, false);
	}
}