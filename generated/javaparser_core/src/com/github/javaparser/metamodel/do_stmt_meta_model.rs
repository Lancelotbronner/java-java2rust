use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::stmt::DoStmt;
use java::util::Optional;

pub struct DoStmtMetaModel {
	body_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	condition_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl DoStmtMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::do_stmt_meta_model::DoStmtMetaModel {
		super(super_base_node_meta_model, DoStmt.class, "DoStmt", "com.github.javaparser.ast.stmt", false, false);
	}
}