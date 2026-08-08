use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::comments::BlockComment;
use java::util::Optional;

pub struct BlockCommentMetaModel;

impl BlockCommentMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::block_comment_meta_model::BlockCommentMetaModel {
		super(super_base_node_meta_model, BlockComment.class, "BlockComment", "com.github.javaparser.ast.comments", false, false);
	}
}