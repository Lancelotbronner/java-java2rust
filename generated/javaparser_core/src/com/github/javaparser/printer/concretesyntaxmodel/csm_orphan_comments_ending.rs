use crate::com::github::javaparser::utils::PositionUtils::sortByBeginPosition;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::comments::Comment;
use crate::com::github::javaparser::printer::SourcePrinter;
use java::util::LinkedList;
use java::util::List;

pub struct CsmOrphanCommentsEnding;

impl CsmOrphanCommentsEnding {
	pub fn pretty_print(&self, node: &com::github::javaparser::ast::node::Node, printer: &com::github::javaparser::printer::source_printer::SourcePrinter) {
		let everything: List<Node> = LinkedList<>::new();
		everything.addAll(&node.get_child_nodes());
		com::github::javaparser::utils::position_utils::PositionUtils::sort_by_begin_position(everything);
		if everything.isEmpty() {
			return;
		}
		let comments_at_end: i32 = 0;
		let finding_comments: bool = true;
		while finding_comments && comments_at_end < everything.size() {
			let last: Node = everything.get(everything.size() - 1 - comments_at_end);
			finding_comments = (last instanceof Comment);
			if finding_comments {
				comments_at_end += 1;
			}
		}
		 {
			let i: i32 = 0;
			while i < comments_at_end {
				{
					let c: Comment = everything.get(everything.size() - comments_at_end + i) as Comment;
					CsmComment::process(c, printer);
				}
				i += 1;
			 }
		 }
	
	}
}

impl com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement for CsmOrphanCommentsEnding {}