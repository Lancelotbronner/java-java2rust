use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::stmt::ReturnStmt;
use java::util::Optional;

pub struct ReturnStmtMetaModel {
	expression_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ReturnStmtMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::return_stmt_meta_model::ReturnStmtMetaModel {
		super(super_base_node_meta_model, ReturnStmt.class, "ReturnStmt", "com.github.javaparser.ast.stmt", false, false);
	}
}