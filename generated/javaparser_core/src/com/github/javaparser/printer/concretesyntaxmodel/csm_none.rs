use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::printer::SourcePrinter;

pub struct CsmNone;

impl CsmNone {
	pub fn pretty_print(&self, node: &com::github::javaparser::ast::node::Node, printer: &com::github::javaparser::printer::source_printer::SourcePrinter) {
	}
}

impl com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement for CsmNone {}