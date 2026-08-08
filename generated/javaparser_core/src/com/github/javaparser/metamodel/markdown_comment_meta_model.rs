use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::comments::MarkdownComment;
use java::util::Optional;

pub struct MarkdownCommentMetaModel;

impl MarkdownCommentMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::markdown_comment_meta_model::MarkdownCommentMetaModel {
		super(super_base_node_meta_model, MarkdownComment.class, "MarkdownComment", "com.github.javaparser.ast.comments", false, false);
	}
}