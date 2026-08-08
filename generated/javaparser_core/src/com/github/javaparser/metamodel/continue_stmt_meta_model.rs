use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::stmt::ContinueStmt;
use java::util::Optional;

pub struct ContinueStmtMetaModel {
	label_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ContinueStmtMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::continue_stmt_meta_model::ContinueStmtMetaModel {
		super(super_base_node_meta_model, ContinueStmt.class, "ContinueStmt", "com.github.javaparser.ast.stmt", false, false);
	}
}