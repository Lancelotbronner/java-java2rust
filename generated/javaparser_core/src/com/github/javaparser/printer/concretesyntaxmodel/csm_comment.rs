use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::comments::Comment;
use crate::com::github::javaparser::printer::SourcePrinter;

pub struct CsmComment;

impl CsmComment {
	fn process(&self, comment: &com::github::javaparser::ast::comments::comment::Comment, printer: &com::github::javaparser::printer::source_printer::SourcePrinter) {
		let content: String = printer.normalize_eol_in_text_block(&comment.get_content());
		printer.print(&comment.get_header());
		printer.print(content);
		printer.println(&comment.get_footer());
	}

	pub fn pretty_print(&self, node: &com::github::javaparser::ast::node::Node, printer: &com::github::javaparser::printer::source_printer::SourcePrinter) {
		node.get_comment().ifPresent(|c|com::github::javaparser::printer::concretesyntaxmodel::csm_comment::CsmComment::process(c, printer));
	}
}

impl com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement for CsmComment {}