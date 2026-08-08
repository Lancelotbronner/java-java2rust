use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::observer::ObservableProperty;

pub struct NoChange;

impl NoChange {
	pub fn get_value(&self, property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.RuntimeException) */ -> /* Java */ java::lang::Object /**/ {
		return property.get_raw_value(node)?;
	}

	pub fn get_property(&self) -> com::github::javaparser::ast::observer::observable_property::ObservableProperty {
		return null;
	}
}

impl com::github::javaparser::printer::lexicalpreservation::changes::change::Change for NoChange {}