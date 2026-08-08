use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::printer::SourcePrinter;

pub struct CsmTextBlock {
	property: com::github::javaparser::ast::observer::observable_property::ObservableProperty,
}

impl CsmTextBlock {
	pub fn new(property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty) -> com::github::javaparser::printer::concretesyntaxmodel::csm_text_block::CsmTextBlock {
		self.property = property;
	}

	pub fn get_property(&self) -> com::github::javaparser::ast::observer::observable_property::ObservableProperty {
		return self.property;
	}

	pub fn pretty_print(&self, node: &com::github::javaparser::ast::node::Node, printer: &com::github::javaparser::printer::source_printer::SourcePrinter) /* thrown(java.lang.RuntimeException) */ {
		printer.print("\"\"\"");
		// Per https://openjdk.java.net/jeps/378#1--Line-terminators, any 'CRLF' and 'CR' are turned into 'LF' before
		// interpreting the text
		printer.println();
		// TODO: Confirm if we need to force this to use {@code \n} separators
		printer.print(&self.property.get_value_as_string_attribute(node)?);
		printer.print("\"\"\"");
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return String::format("%s(property:%s)", &self.getClass().getSimpleName(), &self.get_property());
	}
}

impl com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement for CsmTextBlock {}