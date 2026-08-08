use crate::com::github::javaparser::ast::ImportDeclaration;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithName;
use crate::com::github::javaparser::printer::configuration::ImportOrderingStrategy;
use java::util::Arrays;
use java::util::Comparator;
use java::util::List;

pub struct EclipseImportOrderingStrategy {
	sort_imports_alphabetically: bool = false,
}

impl EclipseImportOrderingStrategy {
	pub fn sort_imports(&self, nodes: &com::github::javaparser::ast::node_list::NodeList) -> /* Java */ java::util::List /**/ {
		let static_imports: NodeList<ImportDeclaration> = NodeList<>::new();
		let java_imports: NodeList<ImportDeclaration> = NodeList<>::new();
		let java_x_imports: NodeList<ImportDeclaration> = NodeList<>::new();
		let org_imports: NodeList<ImportDeclaration> = NodeList<>::new();
		let com_imports: NodeList<ImportDeclaration> = NodeList<>::new();
		let other_imports: NodeList<ImportDeclaration> = NodeList<>::new();
		for import_declaration in nodes {
			// Check if is a static import
			if import_declaration.is_static() {
				static_imports.add(import_declaration);
				continue;
			}
			let import_name: String = import_declaration.get_name_as_string();
			if import_name.startsWith("java.") {
				java_imports.add(import_declaration);
			} else if import_name.startsWith("javax.") {
				java_x_imports.add(import_declaration);
			} else if import_name.startsWith("org.") {
				org_imports.add(import_declaration);
			} else if import_name.startsWith("com.") {
				com_imports.add(import_declaration);
			} else {
				other_imports.add(import_declaration);
			}
		}
		if self.sort_imports_alphabetically {
			let sort_logic: Comparator<ImportDeclaration> = Comparator::comparing(NodeWithName::getNameAsString);
			static_imports.sort(sort_logic);
			java_imports.sort(sort_logic);
			java_x_imports.sort(sort_logic);
			org_imports.sort(sort_logic);
			com_imports.sort(sort_logic);
			other_imports.sort(sort_logic);
		}
		return Arrays::asList(static_imports, java_imports, java_x_imports, org_imports, com_imports, other_imports);
	}

	pub fn set_sort_imports_alphabetically(&mut self, sort_alphabetically: bool) {
		self.sort_imports_alphabetically = sort_alphabetically;
	}

	pub fn is_sort_imports_alphabetically(&self) -> bool {
		return self.sort_imports_alphabetically;
	}
}

impl com::github::javaparser::printer::configuration::import_ordering_strategy::ImportOrderingStrategy for EclipseImportOrderingStrategy {}