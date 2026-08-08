use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::StringLiteralExpr;
use java::util::Optional;

pub struct StringLiteralExprMetaModel;

impl StringLiteralExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::string_literal_expr_meta_model::StringLiteralExprMetaModel {
		super(super_base_node_meta_model, StringLiteralExpr.class, "StringLiteralExpr", "com.github.javaparser.ast.expr", false, false);
	}
}