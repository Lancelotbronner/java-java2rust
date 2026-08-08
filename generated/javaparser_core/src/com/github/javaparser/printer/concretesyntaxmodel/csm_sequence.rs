use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::printer::SourcePrinter;
use java::util::List;
use java::util::Objects;
use java::util::stream::Collectors;

pub struct CsmSequence {
	elements: /* Java */ java::util::List /**/,
}

impl CsmSequence {
	pub fn new(elements: &/* Java */ java::util::List /**/) /* thrown(java.lang.NullPointerException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::printer::concretesyntaxmodel::csm_sequence::CsmSequence {
		if elements == null {
			return Err(NullPointerException::new());
		}
		if elements.stream().anyMatch(Objects::isNull) {
			return Err(IllegalArgumentException::new("Null element in the sequence"));
		}
		self.elements = elements;
	}

	pub fn get_elements(&self) -> /* Java */ java::util::List /**/ {
		return self.elements;
	}

	pub fn pretty_print(&self, node: &com::github::javaparser::ast::node::Node, printer: &com::github::javaparser::printer::source_printer::SourcePrinter) {
		self.elements.forEach(|e|e.pretty_print(node, printer));
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.elements.stream().map(|e|e.toString()).collect(&Collectors::joining(",", "CsmSequence[", "]"));
	}
}

impl com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement for CsmSequence {}