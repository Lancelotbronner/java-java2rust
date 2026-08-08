use crate::com::github::javaparser::ast::ImportDeclaration;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithName;
use crate::com::github::javaparser::printer::configuration::ImportOrderingStrategy;
use java::util::Arrays;
use java::util::Comparator;
use java::util::List;

pub struct IntelliJImportOrderingStrategy {
	sort_imports_alphabetically: bool = false,
}

impl IntelliJImportOrderingStrategy {
	pub fn sort_imports(&self, nodes: &com::github::javaparser::ast::node_list::NodeList) -> /* Java */ java::util::List /**/ {
		let other_imports: NodeList<ImportDeclaration> = NodeList<>::new();
		let java_imports: NodeList<ImportDeclaration> = NodeList<>::new();
		let static_imports: NodeList<ImportDeclaration> = NodeList<>::new();
		for import_declaration in nodes {
			// Check if is a static import
			if import_declaration.is_static() {
				static_imports.add(import_declaration);
				continue;
			}
			let import_name: String = import_declaration.get_name_as_string();
			if import_name.startsWith("java.") || import_name.startsWith("javax.") {
				java_imports.add(import_declaration);
			} else {
				other_imports.add(import_declaration);
			}
		}
		if self.sort_imports_alphabetically {
			let sort_logic: Comparator<ImportDeclaration> = Comparator::comparing(NodeWithName::getNameAsString);
			other_imports.sort(sort_logic);
			java_imports.sort(sort_logic);
			static_imports.sort(sort_logic);
		}
		return Arrays::asList(other_imports, java_imports, static_imports);
	}

	pub fn set_sort_imports_alphabetically(&mut self, sort_alphabetically: bool) {
		self.sort_imports_alphabetically = sort_alphabetically;
	}

	pub fn is_sort_imports_alphabetically(&self) -> bool {
		return self.sort_imports_alphabetically;
	}
}

impl com::github::javaparser::printer::configuration::import_ordering_strategy::ImportOrderingStrategy for IntelliJImportOrderingStrategy {}