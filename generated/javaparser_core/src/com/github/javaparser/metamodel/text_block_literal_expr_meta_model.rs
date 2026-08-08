use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::expr::TextBlockLiteralExpr;
use java::util::Optional;

pub struct TextBlockLiteralExprMetaModel;

impl TextBlockLiteralExprMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::text_block_literal_expr_meta_model::TextBlockLiteralExprMetaModel {
		super(super_base_node_meta_model, TextBlockLiteralExpr.class, "TextBlockLiteralExpr", "com.github.javaparser.ast.expr", false, false);
	}
}