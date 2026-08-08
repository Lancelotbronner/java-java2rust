use crate::com::github::javaparser::ast::Node::NODE_BY_BEGIN_POSITION;
use java::util::stream::Collectors::toList;
use crate::com::github::javaparser::ast::CompilationUnit;
use crate::com::github::javaparser::ast::Modifier;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::comments::Comment;
use crate::com::github::javaparser::ast::comments::LineComment;
use crate::com::github::javaparser::utils::PositionUtils;
use java::util::Collections;
use java::util::LinkedList;
use java::util::List;
use java::util::TreeSet;

struct CommentsInserter {
	configuration: com::github::javaparser::parser_configuration::ParserConfiguration,
}

impl CommentsInserter {
	fn new(configuration: &com::github::javaparser::parser_configuration::ParserConfiguration) -> com::github::javaparser::comments_inserter::CommentsInserter {
		self.configuration = configuration;
	}

	fn insert_comments(&self, cu: &com::github::javaparser::ast::compilation_unit::CompilationUnit, comments: &/* Java */ java::util::TreeSet /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		if comments.isEmpty() {
			return;
		}
	
		/*  I should sort all the direct children and the comments, if a comment
	        is the first thing then it is a comment to the CompilationUnit */ 
		// FIXME if there is no package it could be also a comment to the following class...
		// so I could use some heuristics in these cases to distinguish the two
		// cases
		let children: List<Node> = cu.get_child_nodes();
		let first_comment: Comment = comments.iterator().next();
		if cu.get_package_declaration().isPresent() && (children.isEmpty() || PositionUtils::are_in_order(first_comment, &cu.get_package_declaration().get())) {
			cu.set_comment(first_comment)?;
			comments.remove(first_comment);
		}
	}

	fn insert_comments(&self, node: &com::github::javaparser::ast::node::Node, comments_to_attribute: &/* Java */ java::util::TreeSet /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		if comments_to_attribute.isEmpty() {
			return;
		}
	
		if node instanceof CompilationUnit {
			self.insert_comments(node as CompilationUnit, comments_to_attribute)?;
		}
		/*  the comment can...
	        1) be inside one of the children, then the comment should be associated to this child
	        2) be outside all children. They could be preceding nothing, a comment or a child.
	           If they preceed a child they are assigned to it, otherwise they remain "orphans"
	        */ 
		// Never attribute comments to modifiers.
		// Never attribute comments to modifiers.
	let 	children: List<Node> = node.get_child_nodes().stream().filter(|n|!(n instanceof Modifier)).collect(&/* Java */ java::util::stream::Collectors /**/::toList());
		let attribute_to_annotation: bool = !(self.configuration.is_ignore_annotations_when_attributing_comments());
		for child in children {
			let comments_inside_child: TreeSet<Comment> = TreeSet<>::new();
			comments_inside_child.addAll(&comments_to_attribute.stream().filter(|comment|comment.has_range()).filter(|comment|PositionUtils::node_contains(child, comment, !attribute_to_annotation)?).collect(&/* Java */ java::util::stream::Collectors /**/::toList()));
			comments_to_attribute.removeAll(comments_inside_child);
			self.insert_comments(child, comments_inside_child)?;
		}
		self.attribute_line_comments_on_same_line(comments_to_attribute, children);
		/*  if a comment is on the line right before a node it should belong
	        to that node*/ 
		if !comments_to_attribute.isEmpty() {
			if self.comment_is_on_next_line(node, &comments_to_attribute.first()) {
				node.set_comment(&comments_to_attribute.first())?;
				comments_to_attribute.remove(&comments_to_attribute.first());
			}
		}
		/*  at this point I create an ordered list of all remaining comments and
	        children */ 
		let previous_comment: Comment = null;
		/* final */ let attributed_comments: List<Comment> = LinkedList<>::new();
		let children_and_comments: List<Node> = LinkedList<>::new();
		// Avoid attributing comments to a meaningless container.
		children_and_comments.addAll(children);
		comments_to_attribute.removeAll(attributed_comments);
		children_and_comments.addAll(comments_to_attribute);
		PositionUtils::sort_by_begin_position(children_and_comments, &self.configuration.is_ignore_annotations_when_attributing_comments());
		for thing in children_and_comments {
			if thing instanceof Comment {
				previous_comment = thing as Comment;
				if !previous_comment.is_orphan() {
					previous_comment = null;
				}
			} else {
				if previous_comment != null && !thing.get_comment().isPresent() {
					if !self.configuration.is_do_not_assign_comments_preceding_empty_lines() || !self.there_are_lines_between(previous_comment, thing) {
						thing.set_comment(previous_comment)?;
						attributed_comments.add(previous_comment);
						previous_comment = null;
					}
				}
			}
		}
		comments_to_attribute.removeAll(attributed_comments);
		// all the remaining are orphan nodes
		for c in comments_to_attribute {
			if c.is_orphan() {
				node.add_orphan_comment(c);
			}
		}
	}

	fn attribute_line_comments_on_same_line(&self, comments_to_attribute: &/* Java */ java::util::TreeSet /**/, children: &/* Java */ java::util::List /**/) {
		/*  I can attribute in line comments to elements preceeding them, if
	        there is something contained in their line */ 
		let attributed_comments: List<Comment> = LinkedList<>::new();
		comments_to_attribute.stream().filter(|comment|comment.has_range()).filter(Comment::isLineComment).forEach(|comment|children.stream().filter(|child|child.has_range()).forEach(|child|{
			let comment_range: Range = comment.get_range().get();
			let child_range: Range = child.get_range().get();
			if child_range.end.line == comment_range.begin.line && self.attribute_line_comment_to_node_or_child(child, &comment.as_line_comment()?) {
				attributed_comments.add(comment);
			}
		}));
		comments_to_attribute.removeAll(attributed_comments);
	}

	fn attribute_line_comment_to_node_or_child(&self, node: &com::github::javaparser::ast::node::Node, line_comment: &com::github::javaparser::ast::comments::line_comment::LineComment) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		if !node.has_range() || !line_comment.has_range() {
			return false;
		}
		// let's give to it the comment
		if node.get_begin().get().line == line_comment.get_begin().get().line && !node.get_comment().isPresent() {
			if !(node instanceof Comment) {
				node.set_comment(line_comment)?;
			}
			return true;
		}
		// try with all the children, sorted by reverse position (so the
		// first one is the nearest to the comment
		let children: List<Node> = LinkedList<>::new();
		children.addAll(&node.get_child_nodes());
		PositionUtils::sort_by_begin_position(children);
		Collections::reverse(children);
		for child in children {
			if self.attribute_line_comment_to_node_or_child(child, line_comment)? {
				return true;
			}
		}
		return false;
	}

	fn there_are_lines_between(&self, a: &com::github::javaparser::ast::node::Node, b: &com::github::javaparser::ast::node::Node) -> bool {
		if !a.has_range() || !b.has_range() {
			return true;
		}
		if !PositionUtils::are_in_order(a, b) {
			return self.there_are_lines_between(b, a);
		}
		let end_of_a: i32 = a.get_end().get().line;
		return b.get_begin().get().line > end_of_a + 1;
	}

	fn comment_is_on_next_line(&self, a: &com::github::javaparser::ast::node::Node, c: &com::github::javaparser::ast::comments::comment::Comment) -> bool {
		if !c.has_range() || !a.has_range() {
			return false;
		}
	
		return c.get_range().get().end.line + 1 == a.get_range().get().begin.line;
	}
}