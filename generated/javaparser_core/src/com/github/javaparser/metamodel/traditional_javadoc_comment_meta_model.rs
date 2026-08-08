use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::comments::TraditionalJavadocComment;
use java::util::Optional;

pub struct TraditionalJavadocCommentMetaModel;

impl TraditionalJavadocCommentMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::traditional_javadoc_comment_meta_model::TraditionalJavadocCommentMetaModel {
		super(super_base_node_meta_model, TraditionalJavadocComment.class, "TraditionalJavadocComment", "com.github.javaparser.ast.comments", false, false);
	}
}