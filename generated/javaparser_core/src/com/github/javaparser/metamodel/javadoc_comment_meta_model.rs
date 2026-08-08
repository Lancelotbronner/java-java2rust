use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::comments::JavadocComment;
use java::util::Optional;

pub struct JavadocCommentMetaModel;

impl JavadocCommentMetaModel {
	fn new(super_base_node_meta_model: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::metamodel::javadoc_comment_meta_model::JavadocCommentMetaModel {
		super(super_base_node_meta_model, JavadocComment.class, "JavadocComment", "com.github.javaparser.ast.comments", true, false);
	}

	fn new(super_node_meta_model: &/* Java */ java::util::Optional /**/, type: &/* Java */ java::lang::Class /**/, name: &/* Java */ java::lang::String /**/, package_name: &/* Java */ java::lang::String /**/, is_abstract: bool, has_wildcard: bool) -> com::github::javaparser::metamodel::javadoc_comment_meta_model::JavadocCommentMetaModel {
		super(super_node_meta_model, type, name, package_name, is_abstract, has_wildcard);
	}
}