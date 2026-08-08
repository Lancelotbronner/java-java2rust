use crate::com::github::javaparser::utils::CodeGenerationUtils::f;
use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::metamodel::CommentMetaModel;
use crate::com::github::javaparser::metamodel::InternalProperty;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct Comment {
	content: /* Java */ java::lang::String /**/,
	commented_node: com::github::javaparser::ast::node::Node,
}

impl Comment {
	pub fn new(content: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::comments::comment::Comment {
		this(null, content);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, content: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::comments::comment::Comment {
		super(token_range);
		self.set_content(content);
		self.custom_initialization();
	}

	pub fn get_content(&self) -> /* Java */ java::lang::String /**/ {
		return self.content;
	}

	pub fn set_content(&mut self, content: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::comments::comment::Comment {
		com::github::javaparser::utils::utils::Utils::assert_not_null(content)?;
		if content.equals(self.content) {
			return self;
		}
		self.notify_property_change(ObservableProperty::CONTENT, self.content, content);
		self.content = content;
		return self;
	}

	pub fn is_line_comment(&self) -> bool {
		return false;
	}

	pub fn as_line_comment(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::comments::line_comment::LineComment {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not LineComment, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn get_commented_node(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.commentedNode);
	}

	pub fn set_commented_node(&mut self, commented_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::comments::comment::Comment {
		self.notify_property_change(ObservableProperty::COMMENTED_NODE, self.commentedNode, commented_node);
		if commented_node == null {
			self.commentedNode = null;
			return self;
		}
		if commented_node == self {
			return Err(IllegalArgumentException::new());
		}
		if commented_node instanceof Comment {
			return Err(IllegalArgumentException::new());
		}
		self.commentedNode = commented_node;
		return self;
	}

	pub fn is_orphan(&self) -> bool {
		return self.commentedNode == null;
	}

	pub fn set_comment(&self, comment: &com::github::javaparser::ast::comments::comment::Comment) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::node::Node {
		// comments on comments are not allowed, so we override setComment(Comment) here
		if comment != null {
			return Err(IllegalArgumentException::new("A comment cannot be commented."));
		}
		return super.set_comment(comment);
	}

	pub fn remove(&self) -> bool {
		if self.commentedNode != null {
			self.commentedNode.set_comment(null);
			return true;
		}
		if self.get_parent_node().isPresent() {
			return self.get_parent_node().get().remove_orphan_comment(self);
		}
		return false;
	}

	pub fn find_root_node(&self) -> com::github::javaparser::ast::node::Node {
		// (Non-orphan) comments are not integrated into the normal AST; we need to get the commented node first.
		let n: Node = self.get_commented_node().orElse(self);
		while n.get_parent_node().isPresent() {
			n = n.get_parent_node().get();
		}
		return n;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::comments::comment::Comment {
		return self.accept(CloneVisitor::new(), null) as Comment;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::comment_meta_model::CommentMetaModel {
		return JavaParserMetaModel::commentMetaModel;
	}

	pub fn is_block_comment(&self) -> bool {
		return false;
	}

	pub fn as_block_comment(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::comments::block_comment::BlockComment {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not BlockComment, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn is_javadoc_comment(&self) -> bool {
		return false;
	}

	pub fn as_javadoc_comment(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::comments::javadoc_comment::JavadocComment {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not JavadocComment, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn if_block_comment(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_javadoc_comment(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn if_line_comment(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn to_block_comment(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_javadoc_comment(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn to_line_comment(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn get_header(&self) -> /* Java */ java::lang::String /**/ ;

	pub fn get_footer(&self) -> /* Java */ java::lang::String /**/ ;

	pub fn as_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.get_header() + self.get_content() + self.get_footer();
	}

	pub fn is_markdown_comment(&self) -> bool {
		return false;
	}

	pub fn as_markdown_comment(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::comments::markdown_comment::MarkdownComment {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not MarkdownComment, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn to_markdown_comment(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn if_markdown_comment(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}

	pub fn is_traditional_javadoc_comment(&self) -> bool {
		return false;
	}

	pub fn as_traditional_javadoc_comment(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::comments::traditional_javadoc_comment::TraditionalJavadocComment {
		return Err(IllegalStateException::new(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("%s is not TraditionalJavadocComment, it is %s", self, &self.getClass().getSimpleName())));
	}

	pub fn to_traditional_javadoc_comment(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::empty();
	}

	pub fn if_traditional_javadoc_comment(&self, action: &/* Java */ java::util::function::Consumer /**/) {
	}
}

impl /* Java */ java::lang::Cloneable /**/ for Comment {}

impl com::github::javaparser::has_parent_node::HasParentNode for Comment {}

impl com::github::javaparser::ast::observer::observable::Observable for Comment {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for Comment {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for Comment {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for Comment {}