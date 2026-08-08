use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::stmt::ExplicitConstructorInvocationStmt;
use java::util::Optional;

pub struct ExplicitConstructorInvocationStmtMetaModel {
	arguments_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	expression_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	is_this_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	type_arguments_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
	using_diamond_operator_property_meta_model: com::github::javaparser::metamodel::property_meta_model::PropertyMetaModel,
}

impl ExplicitConstructorInvocationStmtMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::explicit_constructor_invocation_stmt_meta_model::ExplicitConstructorInvocationStmtMetaModel {
		super(super_base_node_meta_model, ExplicitConstructorInvocationStmt.class, "ExplicitConstructorInvocationStmt", "com.github.javaparser.ast.stmt", false, false);
	}
}