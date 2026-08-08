use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::printer::SourcePrinter;
use java::util::List;
use java::util::Objects;
use java::util::stream::Collectors;

pub struct CsmMix {
	elements: /* Java */ java::util::List /**/,
}

impl CsmMix {
	pub fn new(elements: &/* Java */ java::util::List /**/) /* thrown(java.lang.NullPointerException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::printer::concretesyntaxmodel::csm_mix::CsmMix {
		if elements == null {
			return Err(NullPointerException::new());
		}
		if elements.stream().anyMatch(Objects::isNull) {
			return Err(IllegalArgumentException::new("Null element in the mix"));
		}
		self.elements = elements;
	}

	pub fn get_elements(&self) -> /* Java */ java::util::List /**/ {
		return self.elements;
	}

	pub fn pretty_print(&self, node: &com::github::javaparser::ast::node::Node, printer: &com::github::javaparser::printer::source_printer::SourcePrinter) {
		self.elements.forEach(|e|e.pretty_print(node, printer));
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if o == null || self.getClass() != o.getClass() {
			return false;
		}
	
		let csm_mix: CsmMix = o as CsmMix;
		return  if self.elements != null { self.elements.equals(csm_mix.elements) } else { csm_mix.elements == null };
	}

	pub fn hash_code(&self) -> i32 {
		return  if self.elements != null { self.elements.hashCode() } else { 0 };
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.elements.stream().map(|e|e.toString()).collect(&Collectors::joining(",", "CsmMix[", "]"));
	}
}

impl com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement for CsmMix {}