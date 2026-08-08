use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;

pub struct PropagatingAstObserver;

impl PropagatingAstObserver {
	pub fn transform_in_propagating_observer(&self, observer: &com::github::javaparser::ast::observer::ast_observer::AstObserver) -> com::github::javaparser::ast::observer::propagating_ast_observer::PropagatingAstObserver {
		if observer instanceof PropagatingAstObserver {
			return observer as PropagatingAstObserver;
		}
		return PropagatingAstObserver::new() {
			pub fn concrete_property_change(&self, observed_node: &Node, property: &ObservableProperty, old_value: &Object, new_value: &Object) {
				observer.property_change(observed_node, property, old_value, new_value);
			}
	
			pub fn concrete_list_change(&self, observed_node: &NodeList<?>, type: &ListChangeType, index: i32, node_added_or_removed: &Node) {
				observer.list_change(observed_node, type, index, node_added_or_removed);
			}
	
			pub fn parent_change(&self, observed_node: &Node, previous_parent: &Node, new_parent: &Node) {
				observer.parent_change(observed_node, previous_parent, new_parent);
			}
	
		};
	}

	pub fn concrete_property_change(&self, observed_node: &com::github::javaparser::ast::node::Node, property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, old_value: &/* Java */ java::lang::Object /**/, new_value: &/* Java */ java::lang::Object /**/) {
		observer.property_change(observed_node, property, old_value, new_value);
	}

	pub fn concrete_list_change(&self, observed_node: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::observer::ast_observer::ListChangeType, index: i32, node_added_or_removed: &com::github::javaparser::ast::node::Node) {
		observer.list_change(observed_node, type, index, node_added_or_removed);
	}

	pub fn parent_change(&self, observed_node: &com::github::javaparser::ast::node::Node, previous_parent: &com::github::javaparser::ast::node::Node, new_parent: &com::github::javaparser::ast::node::Node) {
		observer.parent_change(observed_node, previous_parent, new_parent);
	}

	pub fn property_change(&self, observed_node: &com::github::javaparser::ast::node::Node, property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, old_value: &/* Java */ java::lang::Object /**/, new_value: &/* Java */ java::lang::Object /**/) {
		self.consider_removing(old_value);
		self.consider_adding(new_value);
		self.concrete_property_change(observed_node, property, old_value, new_value);
	}

	pub fn list_change(&self, observed_node: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::observer::ast_observer::ListChangeType, index: i32, node_added_or_removed: &com::github::javaparser::ast::node::Node) {
		if type == ListChangeType::REMOVAL {
			self.consider_removing(node_added_or_removed);
		} else if type == ListChangeType::ADDITION {
			self.consider_adding(node_added_or_removed);
		}
		self.concrete_list_change(observed_node, type, index, node_added_or_removed);
	}

	pub fn list_replacement(&self, observed_node: &com::github::javaparser::ast::node_list::NodeList, index: i32, old_node: &com::github::javaparser::ast::node::Node, new_node: &com::github::javaparser::ast::node::Node) {
		if old_node == new_node {
			return;
		}
		self.consider_removing(old_node);
		self.consider_adding(new_node);
		self.concrete_list_replacement(observed_node, index, old_node, new_node);
	}

	pub fn concrete_property_change(&self, observed_node: &com::github::javaparser::ast::node::Node, property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, old_value: &/* Java */ java::lang::Object /**/, new_value: &/* Java */ java::lang::Object /**/) {
	// do nothing
	}

	pub fn concrete_list_change(&self, observed_node: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::observer::ast_observer::ListChangeType, index: i32, node_added_or_removed: &com::github::javaparser::ast::node::Node) {
	// do nothing
	}

	pub fn concrete_list_replacement(&self, observed_node: &com::github::javaparser::ast::node_list::NodeList, index: i32, old_value: &com::github::javaparser::ast::node::Node, new_value: &com::github::javaparser::ast::node::Node) {
	// do nothing
	}

	pub fn parent_change(&self, observed_node: &com::github::javaparser::ast::node::Node, previous_parent: &com::github::javaparser::ast::node::Node, new_parent: &com::github::javaparser::ast::node::Node) {
	// do nothing
	}

	fn consider_removing(&self, element: &/* Java */ java::lang::Object /**/) {
		if element instanceof Observable {
			if (element as Observable).is_registered(self) {
				(element as Observable).unregister(self);
			}
		}
	}

	fn consider_adding(&self, element: &/* Java */ java::lang::Object /**/) {
		if element instanceof Node {
			(element as Node).register_for_subtree(self);
		} else if element instanceof Observable {
			(element as Observable).register(self);
		}
	}
}

impl com::github::javaparser::ast::observer::ast_observer::AstObserver for PropagatingAstObserver {}