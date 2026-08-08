use crate::com::github::javaparser::ast::Node::NODE_BY_BEGIN_POSITION;
use crate::com::github::javaparser::Range;
use java::util::Collection;
use java::util::Set;
use java::util::TreeSet;
use java::util::stream::Collectors;

pub struct CommentsCollection {
	comments: /* Java */ java::util::TreeSet /**/ = TreeSet<>::new(NODE_BY_BEGIN_POSITION),
}

impl CommentsCollection {
	pub fn new() -> com::github::javaparser::ast::comments::comments_collection::CommentsCollection {
	}

	pub fn new(comments_to_copy: &/* Java */ java::util::Collection /**/) -> com::github::javaparser::ast::comments::comments_collection::CommentsCollection {
		self.comments.addAll(comments_to_copy);
	}

	pub fn get_line_comments(&self) -> /* Java */ java::util::Set /**/ {
		return self.comments.stream().filter(|comment|comment instanceof LineComment).map(|comment|comment as LineComment).collect(&Collectors::toCollection(|()|TreeSet<>::new()));
	}

	pub fn get_block_comments(&self) -> /* Java */ java::util::Set /**/ {
		return self.comments.stream().filter(|comment|comment instanceof BlockComment).map(|comment|comment as BlockComment).collect(&Collectors::toCollection(|()|TreeSet<>::new()));
	}

	pub fn get_javadoc_comments(&self) -> /* Java */ java::util::Set /**/ {
		return self.comments.stream().filter(|comment|comment instanceof JavadocComment).map(|comment|comment as JavadocComment).collect(&Collectors::toCollection(|()|TreeSet<>::new()));
	}

	pub fn add_comment(&self, comment: &com::github::javaparser::ast::comments::comment::Comment) {
		self.comments.add(comment);
	}

	pub fn contains(&self, comment: &com::github::javaparser::ast::comments::comment::Comment) -> bool {
		if !comment.has_range() {
			return false;
		}
		let comment_range: Range = comment.get_range().get();
		for c in self.get_comments() {
			if !c.has_range() {
				return false;
			}
			let c_range: Range = c.get_range().get();
			// it depends how \r and \n are calculated...
			if c_range.begin.equals(comment_range.begin) && c_range.end.line == comment_range.end.line && Math::abs(c_range.end.column - comment_range.end.column) < 2 {
				return true;
			}
		}
		return false;
	}

	pub fn get_comments(&self) -> /* Java */ java::util::TreeSet /**/ {
		return self.comments;
	}

	pub fn size(&self) -> i32 {
		return self.comments.size();
	}

	pub fn minus(&self, other: &com::github::javaparser::ast::comments::comments_collection::CommentsCollection) -> com::github::javaparser::ast::comments::comments_collection::CommentsCollection {
		let result: CommentsCollection = CommentsCollection::new();
		result.comments.addAll(&self.comments.stream().filter(|comment|!other.contains(comment)).collect(&Collectors::toList()));
		return result;
	}

	pub fn copy(&self) -> com::github::javaparser::ast::comments::comments_collection::CommentsCollection {
		return CommentsCollection::new(self.comments);
	}
}