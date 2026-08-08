use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::stmt::YieldStmt;
use java::util::Optional;

pub struct YieldStmtMetaModel {
	expression_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl YieldStmtMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::yield_stmt_meta_model::YieldStmtMetaModel {
		super(super_base_node_meta_model, YieldStmt.class, "YieldStmt", "com.github.javaparser.ast.stmt", false, false);
	}
}