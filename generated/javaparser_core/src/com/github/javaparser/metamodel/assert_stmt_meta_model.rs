use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::stmt::AssertStmt;
use java::util::Optional;

pub struct AssertStmtMetaModel {
	check_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	message_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl AssertStmtMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::assert_stmt_meta_model::AssertStmtMetaModel {
		super(super_base_node_meta_model, AssertStmt.class, "AssertStmt", "com.github.javaparser.ast.stmt", false, false);
	}
}