use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::stmt::UnparsableStmt;
use java::util::Optional;

pub struct UnparsableStmtMetaModel;

impl UnparsableStmtMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::unparsable_stmt_meta_model::UnparsableStmtMetaModel {
		super(super_base_node_meta_model, UnparsableStmt.class, "UnparsableStmt", "com.github.javaparser.ast.stmt", false, false);
	}
}