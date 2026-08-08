use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::stmt::BlockStmt;
use java::util::Optional;

pub struct BlockStmtMetaModel {
	statements_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl BlockStmtMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::block_stmt_meta_model::BlockStmtMetaModel {
		super(super_base_node_meta_model, BlockStmt.class, "BlockStmt", "com.github.javaparser.ast.stmt", false, false);
	}
}