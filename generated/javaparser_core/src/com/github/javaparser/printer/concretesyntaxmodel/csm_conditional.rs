use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::printer::SourcePrinter;
use java::util::Arrays;
use java::util::List;

pub struct CsmConditional {
	condition: com::github::javaparser::printer::concretesyntaxmodel::csm_conditional::Condition,
	properties: /* Java */ java::util::List /**/,
	then_element: com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement,
	else_element: com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement,
}

impl CsmConditional {
	pub fn get_condition(&self) -> com::github::javaparser::printer::concretesyntaxmodel::csm_conditional::Condition {
		return self.condition;
	}

	pub fn get_property(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::observer::observable_property::ObservableProperty {
		if self.properties.size() > 1 {
			return Err(IllegalStateException::new());
		}
		return self.properties.get(0);
	}

	pub fn get_properties(&self) -> /* Java */ java::util::List /**/ {
		return self.properties;
	}

	pub fn get_then_element(&self) -> com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement {
		return self.then_element;
	}

	pub fn get_else_element(&self) -> com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement {
		return self.else_element;
	}

	pub fn new(property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, condition: &com::github::javaparser::printer::concretesyntaxmodel::csm_conditional::Condition, then_element: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement, else_element: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement) -> com::github::javaparser::printer::concretesyntaxmodel::csm_conditional::CsmConditional {
		self.properties = Arrays::asList(property);
		self.condition = condition;
		self.thenElement = then_element;
		self.elseElement = else_element;
	}

	pub fn new(properties: &/* Java */ java::util::List /**/, condition: &com::github::javaparser::printer::concretesyntaxmodel::csm_conditional::Condition, then_element: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement, else_element: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::printer::concretesyntaxmodel::csm_conditional::CsmConditional {
		if properties.size() < 1 {
			return Err(IllegalArgumentException::new());
		}
		self.properties = properties;
		self.condition = condition;
		self.thenElement = then_element;
		self.elseElement = else_element;
	}

	pub fn new(property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, condition: &com::github::javaparser::printer::concretesyntaxmodel::csm_conditional::Condition, then_element: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement) -> com::github::javaparser::printer::concretesyntaxmodel::csm_conditional::CsmConditional {
		this(property, condition, then_element, CsmNone::new());
	}

	pub fn pretty_print(&self, node: &com::github::javaparser::ast::node::Node, printer: &com::github::javaparser::printer::source_printer::SourcePrinter) {
		let test: bool = false;
		for prop in self.properties {
			test = test || self.condition.evaluate(node, prop);
		}
		if test {
			self.then_element.pretty_print(node, printer);
		} else {
			self.else_element.pretty_print(node, printer);
		}
	}
}

impl com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement for CsmConditional {}

pub enum Condition;