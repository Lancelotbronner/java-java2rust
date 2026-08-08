use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::printer::ConcreteSyntaxModel;
use crate::com::github::javaparser::printer::SourcePrinter;

pub struct CsmSingleReference {
	property: com::github::javaparser::ast::observer::observable_property::ObservableProperty,
}

impl CsmSingleReference {
	pub fn get_property(&self) -> com::github::javaparser::ast::observer::observable_property::ObservableProperty {
		return self.property;
	}

	pub fn new(property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty) -> com::github::javaparser::printer::concretesyntaxmodel::csm_single_reference::CsmSingleReference {
		self.property = property;
	}

	pub fn pretty_print(&self, node: &com::github::javaparser::ast::node::Node, printer: &com::github::javaparser::printer::source_printer::SourcePrinter) {
		let child: Node = self.property.get_value_as_single_reference(node)?;
		if child != null {
			ConcreteSyntaxModel::generic_pretty_print(child, printer);
		}
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return String::format("%s(property:%s)", &self.getClass().getSimpleName(), &self.get_property());
	}
}

impl com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement for CsmSingleReference {}