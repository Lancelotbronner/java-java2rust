use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::observer::ObservableProperty;

pub struct PropertyChange {
	property: com::github::javaparser::ast::observer::observable_property::ObservableProperty,
	old_value: /* Java */ java::lang::Object /**/,
	new_value: /* Java */ java::lang::Object /**/,
}

impl PropertyChange {
	pub fn new(property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, old_value: &/* Java */ java::lang::Object /**/, new_value: &/* Java */ java::lang::Object /**/) -> com::github::javaparser::printer::lexicalpreservation::changes::property_change::PropertyChange {
		self.property = property;
		self.oldValue = old_value;
		self.newValue = new_value;
	}

	pub fn get_property(&self) -> com::github::javaparser::ast::observer::observable_property::ObservableProperty {
		return self.property;
	}

	pub fn get_old_value(&self) -> /* Java */ java::lang::Object /**/ {
		return self.old_value;
	}

	pub fn get_new_value(&self) -> /* Java */ java::lang::Object /**/ {
		return self.new_value;
	}

	pub fn get_value(&self, property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.RuntimeException) */ -> /* Java */ java::lang::Object /**/ {
		if property == self.property {
			return self.new_value;
		}
		return property.get_raw_value(node)?;
	}
}

impl com::github::javaparser::printer::lexicalpreservation::changes::change::Change for PropertyChange {}