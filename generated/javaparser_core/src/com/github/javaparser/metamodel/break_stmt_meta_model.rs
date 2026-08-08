use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::stmt::BreakStmt;
use java::util::Optional;

pub struct BreakStmtMetaModel {
	label_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl BreakStmtMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::break_stmt_meta_model::BreakStmtMetaModel {
		super(super_base_node_meta_model, BreakStmt.class, "BreakStmt", "com.github.javaparser.ast.stmt", false, false);
	}
}