use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::stmt::EmptyStmt;
use java::util::Optional;

pub struct EmptyStmtMetaModel;

impl EmptyStmtMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::empty_stmt_meta_model::EmptyStmtMetaModel {
		super(super_base_node_meta_model, EmptyStmt.class, "EmptyStmt", "com.github.javaparser.ast.stmt", false, false);
	}
}