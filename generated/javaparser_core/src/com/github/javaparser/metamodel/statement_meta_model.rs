use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::stmt::Statement;
use java::util::Optional;

pub struct StatementMetaModel;

impl StatementMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::statement_meta_model::StatementMetaModel {
		super(super_base_node_meta_model, Statement.class, "Statement", "com.github.javaparser.ast.stmt", true, false);
	}

	fn new(super_node_meta_model: &/* Java */ java::util::Optional /**/, type: &/* Java */ java::lang::Class /**/, name: &/* Java */ java::lang::String /**/, package_name: &/* Java */ java::lang::String /**/, is_abstract: bool, has_wildcard: bool) -> com::github::javaparser::metamodel::statement_meta_model::StatementMetaModel {
		super(super_node_meta_model, type, name, package_name, is_abstract, has_wildcard);
	}
}