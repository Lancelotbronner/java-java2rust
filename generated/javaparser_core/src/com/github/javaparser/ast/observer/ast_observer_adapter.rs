use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;

pub struct AstObserverAdapter;

impl AstObserverAdapter {
	pub fn property_change(&self, observed_node: &com::github::javaparser::ast::node::Node, property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, old_value: &/* Java */ java::lang::Object /**/, new_value: &/* Java */ java::lang::Object /**/) {
	// do nothing
	}

	pub fn parent_change(&self, observed_node: &com::github::javaparser::ast::node::Node, previous_parent: &com::github::javaparser::ast::node::Node, new_parent: &com::github::javaparser::ast::node::Node) {
	// do nothing
	}

	pub fn list_change(&self, observed_node: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::observer::ast_observer::ListChangeType, index: i32, node_added_or_removed: &com::github::javaparser::ast::node::Node) {
	// do nothing
	}

	pub fn list_replacement(&self, observed_node: &com::github::javaparser::ast::node_list::NodeList, index: i32, old_node: &com::github::javaparser::ast::node::Node, new_node: &com::github::javaparser::ast::node::Node) {
	// do nothing
	}
}

impl com::github::javaparser::ast::observer::ast_observer::AstObserver for AstObserverAdapter {}