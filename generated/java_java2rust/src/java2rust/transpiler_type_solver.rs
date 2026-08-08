use javaparser_core::com::github::javaparser::ast::CompilationUnit;
use javaparser_core::com::github::javaparser::ast::Node;
use javaparser_core::com::github::javaparser::ast::body::TypeDeclaration;
use javaparser_core::com::github::javaparser::resolution::TypeSolver;
use javaparser_core::com::github::javaparser::resolution::declarations::ResolvedReferenceTypeDeclaration;
use javaparser_core::com::github::javaparser::resolution::model::SymbolReference;
use crate::java2rust::rust::RustJar;
use crate::java2rust::rust::RustUnit;
use java::util::HashMap;

pub struct TranspilerTypeSolver {
	transpiler: java2rust::java_transpiler::JavaTranspiler,
	types: /* Java */ java::util::HashMap /**/ = HashMap<>::new(),
	parent: com::github::javaparser::resolution::type_solver::TypeSolver,
}

impl TranspilerTypeSolver {
	static UNSOLVED: com::github::javaparser::resolution::model::symbol_reference::SymbolReference = SymbolReference::unsolved();

	pub fn new(transpiler: &java2rust::java_transpiler::JavaTranspiler) -> java2rust::transpiler_type_solver::TranspilerTypeSolver {
		self.transpiler = transpiler;
	}

	pub fn get_parent(&self) -> com::github::javaparser::resolution::type_solver::TypeSolver {
		return self.parent;
	}

	pub fn set_parent(&mut self, parent: &com::github::javaparser::resolution::type_solver::TypeSolver) {
		self.parent = parent;
	}

	pub fn try_to_solve_type(&self, name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::resolution::model::symbol_reference::SymbolReference {
		let result: SymbolReference<ResolvedReferenceTypeDeclaration> = self.types.get(name);
		if result == null {
			return self.UNSOLVED;
		}
	
		return result;
	}

	pub fn try_to_solve_type_in_module(&self, qualified_module_name: &/* Java */ java::lang::String /**/, simple_type_name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::resolution::model::symbol_reference::SymbolReference {
		let id: String = "%s.%s".formatted(qualified_module_name, simple_type_name);
		return self.try_to_solve_type(id);
	}

	pub fn reload(&self) {
		self.types.clear();
		for crate in self.transpiler.crates {
			for unit in crate.units {
				if unit.java.get_result().isEmpty() {
					continue;
				}
	
				let compiled: CompilationUnit = unit.java.get_result().get();
				self.visit(compiled);
			}
		}
	}

	fn visit(&self, node: &com::github::javaparser::ast::node::Node) {
		if node instanceof TypeDeclaration<?> {
			let resolved: ResolvedReferenceTypeDeclaration = /* Java*/ td/* */ .resolve();
			self.types.put(&resolved.get_id(), &SymbolReference::solved(resolved));
		}
		node.get_child_nodes().forEach(self::visit);
	}
}

impl com::github::javaparser::resolution::type_solver::TypeSolver for TranspilerTypeSolver {}