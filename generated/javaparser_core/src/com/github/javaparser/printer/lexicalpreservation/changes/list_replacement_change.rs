use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use java::util::Optional;

pub struct ListReplacementChange {
	observable_property: com::github::javaparser::ast::observer::observable_property::ObservableProperty,
	index: i32,
	new_value: com::github::javaparser::ast::node::Node,
}

impl ListReplacementChange {
	pub fn new(observable_property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, index: i32, new_value: &com::github::javaparser::ast::node::Node) -> com::github::javaparser::printer::lexicalpreservation::changes::list_replacement_change::ListReplacementChange {
		self.observableProperty = observable_property;
		self.index = index;
		self.newValue = new_value;
	}

	pub fn get_value(&self, property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.IllegalArgumentException | java.lang.IllegalStateException) */ -> /* Java */ java::lang::Object /**/ {
		if property == self.observable_property {
			let current_raw_value: Object = NoChange::new().get_value(property, node);
			if current_raw_value instanceof Optional {
				let optional: Optional<?> = current_raw_value as Optional<?>;
				current_raw_value = optional.orElse(null);
			}
			if !(current_raw_value instanceof NodeList) {
				return Err(IllegalStateException::new("Expected NodeList, found " + current_raw_value.getClass().getCanonicalName()));
			}
			let current_node_list: NodeList<Node> = current_raw_value as NodeList<Node>;
			// Note: When adding to a node list children get assigned the list's parent, thus we must set the list's
			// parent before adding children (#2592).
			let new_node_list: NodeList<Node> = NodeList<>::new();
			new_node_list.set_parent_node(&current_node_list.get_parent_node_for_children());
			new_node_list.add_all(current_node_list);
			// Perform modification -- replace an item in the list
			new_node_list.set(self.index, self.new_value)?;
			return new_node_list;
		}
		return NoChange::new().get_value(property, node);
	}

	pub fn get_property(&self) -> com::github::javaparser::ast::observer::observable_property::ObservableProperty {
		return self.observable_property;
	}
}

impl com::github::javaparser::printer::lexicalpreservation::changes::change::Change for ListReplacementChange {}