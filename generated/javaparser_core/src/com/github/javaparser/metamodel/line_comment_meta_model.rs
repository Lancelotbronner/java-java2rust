use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::comments::LineComment;
use java::util::Optional;

pub struct LineCommentMetaModel;

impl LineCommentMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::line_comment_meta_model::LineCommentMetaModel {
		super(super_base_node_meta_model, LineComment.class, "LineComment", "com.github.javaparser.ast.comments", false, false);
	}
}