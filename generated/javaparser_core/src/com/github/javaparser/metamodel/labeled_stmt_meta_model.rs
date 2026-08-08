use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::stmt::LabeledStmt;
use java::util::Optional;

pub struct LabeledStmtMetaModel {
	label_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	statement_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl LabeledStmtMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::labeled_stmt_meta_model::LabeledStmtMetaModel {
		super(super_base_node_meta_model, LabeledStmt.class, "LabeledStmt", "com.github.javaparser.ast.stmt", false, false);
	}
}