use java::util::Comparator::comparingInt;
use crate::com::github::javaparser::ast::ImportDeclaration;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithName;
use crate::com::github::javaparser::printer::configuration::ImportOrderingStrategy;
use java::util::Collections;
use java::util::Comparator;
use java::util::List;

pub struct DefaultImportOrderingStrategy {
	sort_imports_alphabetically: bool = false,
}

impl DefaultImportOrderingStrategy {
	pub fn sort_imports(&self, nodes: &com::github::javaparser::ast::node_list::NodeList) -> /* Java */ java::util::List /**/ {
		if self.sort_imports_alphabetically {
			let sort_logic: Comparator<ImportDeclaration> = /* Java */ java::util::Comparator /**/::comparingInt(|(i: &ImportDeclaration)| if i.is_static() { 0 } else { 1 }).thenComparing(NodeWithName::getNameAsString);
			nodes.sort(sort_logic);
		}
		return Collections::singletonList(nodes);
	}

	pub fn set_sort_imports_alphabetically(&mut self, sort_imports_alphabetically: bool) {
		self.sortImportsAlphabetically = sort_imports_alphabetically;
	}

	pub fn is_sort_imports_alphabetically(&self) -> bool {
		return self.sort_imports_alphabetically;
	}
}

impl com::github::javaparser::printer::configuration::import_ordering_strategy::ImportOrderingStrategy for DefaultImportOrderingStrategy {}