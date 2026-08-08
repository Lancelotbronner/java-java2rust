use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::stmt::LocalClassDeclarationStmt;
use java::util::Optional;

pub struct LocalClassDeclarationStmtMetaModel {
	class_declaration_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl LocalClassDeclarationStmtMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::local_class_declaration_stmt_meta_model::LocalClassDeclarationStmtMetaModel {
		super(super_base_node_meta_model, LocalClassDeclarationStmt.class, "LocalClassDeclarationStmt", "com.github.javaparser.ast.stmt", false, false);
	}
}