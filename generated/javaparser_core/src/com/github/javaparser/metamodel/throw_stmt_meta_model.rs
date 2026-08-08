use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::stmt::ThrowStmt;
use java::util::Optional;

pub struct ThrowStmtMetaModel {
	expression_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ThrowStmtMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::throw_stmt_meta_model::ThrowStmtMetaModel {
		super(super_base_node_meta_model, ThrowStmt.class, "ThrowStmt", "com.github.javaparser.ast.stmt", false, false);
	}
}