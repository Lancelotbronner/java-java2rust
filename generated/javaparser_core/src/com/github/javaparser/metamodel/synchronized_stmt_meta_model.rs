use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::stmt::SynchronizedStmt;
use java::util::Optional;

pub struct SynchronizedStmtMetaModel {
	body_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	expression_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl SynchronizedStmtMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::synchronized_stmt_meta_model::SynchronizedStmtMetaModel {
		super(super_base_node_meta_model, SynchronizedStmt.class, "SynchronizedStmt", "com.github.javaparser.ast.stmt", false, false);
	}
}