use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::printer::SourcePrinter;
use crate::com::github::javaparser::printer::lexicalpreservation::TextElement;
use crate::com::github::javaparser::printer::lexicalpreservation::TokenTextElement;

pub struct CsmIndent;

impl CsmIndent {
	pub fn pretty_print(&self, node: &com::github::javaparser::ast::node::Node, printer: &com::github::javaparser::printer::source_printer::SourcePrinter) /* thrown(java.lang.AssertionError) */ {
		printer.indent()?;
	}

	pub fn is_corresponding_element(&self, text_element: &com::github::javaparser::printer::lexicalpreservation::text_element::TextElement) -> bool {
		return (text_element instanceof TokenTextElement) && (text_element as TokenTextElement).is_space_or_tab();
	}

	pub fn hash_code(&self) -> i32 {
		return 1;
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		return obj instanceof CsmIndent;
	}
}

impl com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement for CsmIndent {}