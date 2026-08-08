use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::stmt::LocalRecordDeclarationStmt;
use java::util::Optional;

pub struct LocalRecordDeclarationStmtMetaModel {
	record_declaration_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl LocalRecordDeclarationStmtMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::local_record_declaration_stmt_meta_model::LocalRecordDeclarationStmtMetaModel {
		super(super_base_node_meta_model, LocalRecordDeclarationStmt.class, "LocalRecordDeclarationStmt", "com.github.javaparser.ast.stmt", false, false);
	}
}