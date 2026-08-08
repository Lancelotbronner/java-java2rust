use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::stmt::SwitchStmt;
use java::util::Optional;

pub struct SwitchStmtMetaModel {
	entries_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	selector_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl SwitchStmtMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::switch_stmt_meta_model::SwitchStmtMetaModel {
		super(super_base_node_meta_model, SwitchStmt.class, "SwitchStmt", "com.github.javaparser.ast.stmt", false, false);
	}
}