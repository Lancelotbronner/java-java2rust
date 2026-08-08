use crate::com::github::javaparser::Range;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use java::util::Optional;

pub struct ListRemovalChange {
	observable_property: com::github::javaparser::ast::observer::observable_property::ObservableProperty,
	index: i32,
}

impl ListRemovalChange {
	pub fn new(observable_property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, index: i32) -> com::github::javaparser::printer::lexicalpreservation::changes::list_removal_change::ListRemovalChange {
		self.observableProperty = observable_property;
		self.index = index;
	}

	pub fn get_value(&self, property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.IllegalStateException) */ -> /* Java */ java::lang::Object /**/ {
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
			// fix #2187 set the parent node in the new list
			new_node_list.set_parent_node(&current_node_list.get_parent_node_for_children());
			// Here we want to obtain a sub-list that does not contain an element.
			// It is important not to implement this by first adding all the elements in the
			// list and then deleting the element to be removed, as this involves event
			// propagation mechanisms, particularly for lexical preservation,
			// which deletes the relationship between a node and its parent node.
			// This relationship is necessary to reinforce indentation, for example when
			// deleting a node, as indentation can be carried by the parent node.
			current_node_list.stream().filter(|n|!self.is_same_node(&current_node_list.get(self.index), n)).forEach(|selected_node|new_node_list.add(selected_node));
			return new_node_list;
		}
		return NoChange::new().get_value(property, node);
	}

	fn is_same_node(&self, n1: &com::github::javaparser::ast::node::Node, n2: &com::github::javaparser::ast::node::Node) -> bool {
		return n1.equals(n2) && self.is_same_range(n1, n2);
	}

	fn is_same_range(&self, n1: &com::github::javaparser::ast::node::Node, n2: &com::github::javaparser::ast::node::Node) -> bool {
		return (!n1.has_range() && !n2.has_range()) || (n1.has_range() && n2.has_range() && self.is_same_range(&n1.get_range().get(), &n2.get_range().get()));
	}

	fn is_same_range(&self, r1: &com::github::javaparser::range::Range, r2: &com::github::javaparser::range::Range) -> bool {
		return r1.equals(r2);
	}

	pub fn get_property(&self) -> com::github::javaparser::ast::observer::observable_property::ObservableProperty {
		return self.observable_property;
	}
}

impl com::github::javaparser::printer::lexicalpreservation::changes::change::Change for ListRemovalChange {}