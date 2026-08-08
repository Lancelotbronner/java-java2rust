use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::printer::ConcreteSyntaxModel;
use crate::com::github::javaparser::printer::SourcePrinter;
use java::util::Collection;
use java::util::Iterator;

pub struct CsmList {
	property: com::github::javaparser::ast::observer::observable_property::ObservableProperty,
	separator_post: com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement,
	separator_pre: com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement,
	preceeding: com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement,
	following: com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement,
}

impl CsmList {
	pub fn get_property(&self) -> com::github::javaparser::ast::observer::observable_property::ObservableProperty {
		return self.property;
	}

	pub fn get_separator_post(&self) -> com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement {
		return self.separator_post;
	}

	pub fn get_separator_pre(&self) -> com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement {
		return self.separator_pre;
	}

	pub fn get_preceeding(&self) -> com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement {
		return self.preceeding;
	}

	pub fn get_following(&self) -> com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement {
		return self.following;
	}

	pub fn new(property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, separator: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement) -> com::github::javaparser::printer::concretesyntaxmodel::csm_list::CsmList {
		this(property, CsmNone::new(), separator, CsmNone::new(), CsmNone::new());
	}

	pub fn new(property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty) -> com::github::javaparser::printer::concretesyntaxmodel::csm_list::CsmList {
		this(property, CsmNone::new(), CsmNone::new(), CsmNone::new(), CsmNone::new());
	}

	pub fn new(property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, separator_pre: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement, separator_post: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement, preceeding: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement, following: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement) -> com::github::javaparser::printer::concretesyntaxmodel::csm_list::CsmList {
		self.property = property;
		self.separatorPre = separator_pre;
		self.separatorPost = separator_post;
		self.preceeding = preceeding;
		self.following = following;
	}

	pub fn pretty_print(&self, node: &com::github::javaparser::ast::node::Node, printer: &com::github::javaparser::printer::source_printer::SourcePrinter) {
		if self.property.is_about_nodes() {
			let node_list: NodeList<? extends Node> = self.property.get_value_as_multiple_reference(node)?;
			if node_list == null {
				return;
			}
			if !node_list.is_empty() && self.preceeding != null {
				self.preceeding.pretty_print(node, printer);
			}
			 {
				let i: i32 = 0;
				while i < node_list.size() {
					{
						if self.separator_pre != null && i != 0 {
							self.separator_pre.pretty_print(node, printer);
						}
						ConcreteSyntaxModel::generic_pretty_print(&node_list.get(i), printer);
						if self.separator_post != null && i != (node_list.size() - 1) {
							self.separator_post.pretty_print(node, printer);
						}
					}
					i += 1;
				 }
			 }
	
			if !node_list.is_empty() && self.following != null {
				self.following.pretty_print(node, printer);
			}
		} else {
			let values: Collection<?> = self.property.get_value_as_collection(node)?;
			if values == null {
				return;
			}
			if !values.isEmpty() && self.preceeding != null {
				self.preceeding.pretty_print(node, printer);
			}
			 {
				let it: Iterator<?> = values.iterator();
				while it.hasNext(){
					if self.separator_pre != null && it.hasNext() {
						self.separator_pre.pretty_print(node, printer);
					}
					printer.print(&PrintingHelper::print_to_string(&it.next()));
					if self.separator_post != null && it.hasNext() {
						self.separator_post.pretty_print(node, printer);
					}
				}
			 }
	
			if !values.isEmpty() && self.following != null {
				self.following.pretty_print(node, printer);
			}
		}
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return String::format("%s(property:%s)", &self.getClass().getSimpleName(), &self.get_property());
	}
}

impl com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement for CsmList {}