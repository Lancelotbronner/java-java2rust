use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::stmt::ExpressionStmt;
use java::util::Optional;

pub struct ExpressionStmtMetaModel {
	expression_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ExpressionStmtMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::expression_stmt_meta_model::ExpressionStmtMetaModel {
		super(super_base_node_meta_model, ExpressionStmt.class, "ExpressionStmt", "com.github.javaparser.ast.stmt", false, false);
	}
}