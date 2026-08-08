use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::stmt::CatchClause;
use java::util::Optional;

pub struct CatchClauseMetaModel {
	body_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	parameter_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl CatchClauseMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::catch_clause_meta_model::CatchClauseMetaModel {
		super(super_base_node_meta_model, CatchClause.class, "CatchClause", "com.github.javaparser.ast.stmt", false, false);
	}
}