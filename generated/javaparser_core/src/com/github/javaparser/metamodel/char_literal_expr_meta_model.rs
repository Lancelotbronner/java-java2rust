use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::CharLiteralExpr;
use java::util::Optional;

pub struct CharLiteralExprMetaModel;

impl CharLiteralExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::char_literal_expr_meta_model::CharLiteralExprMetaModel {
		super(super_base_node_meta_model, CharLiteralExpr.class, "CharLiteralExpr", "com.github.javaparser.ast.expr", false, false);
	}
}