use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::printer::SourcePrinter;

pub struct CsmUnindent;

impl CsmUnindent {
	pub fn pretty_print(&self, node: &com::github::javaparser::ast::node::Node, printer: &com::github::javaparser::printer::source_printer::SourcePrinter) /* thrown(java.lang.IllegalStateException) */ {
		printer.unindent()?;
	}

	pub fn hash_code(&self) -> i32 {
		return 2;
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		return obj instanceof CsmUnindent;
	}
}

impl com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement for CsmUnindent {}