use javaparser_core::com::github::javaparser::ast::Node;
use javaparser_core::com::github::javaparser::ast::body::TypeDeclaration;
use javaparser_core::com::github::javaparser::resolution::TypeSolver;
use javaparser_core::com::github::javaparser::resolution::cache::Cache;
use javaparser_core::com::github::javaparser::resolution::declarations::ResolvedReferenceTypeDeclaration;
use javaparser_core::com::github::javaparser::resolution::model::SymbolReference;
use javaparser_core::com::github::javaparser::symbolsolver::cache::GuavaCache;
use javaparser_core::com::github::javaparser::symbolsolver::javaparsermodel::JavaParserFacade;
use javaparser_core::com::github::javaparser::utils::SourceZip;
use javaparser_core::com::google::common::cache::CacheBuilder;
use commons_lang3::org::apache::commons::lang3::StringUtils;
use java::io::IOException;
use java::nio::file::Path;
use java::util;

pub struct SourceZipTypeSolver {
	sources: com::github::javaparser::utils::source_zip::SourceZip,
	paths: /* Java */ java::util::List /**/ = ArrayList<>::new(),
	types: /* Java */ java::util::HashMap /**/ = HashMap<>::new(),
	common_prefix: /* Java */ java::lang::String /**/ = "",
	parent: com::github::javaparser::resolution::type_solver::TypeSolver,
}

impl SourceZipTypeSolver {
	static CACHE_SIZE_UNSET: i32 = -1;

	pub fn new(sources: &com::github::javaparser::utils::source_zip::SourceZip) -> javaparser::source_zip_type_solver::SourceZipTypeSolver {
		this(sources, self.CACHE_SIZE_UNSET);
	}

	pub fn new(sources: &com::github::javaparser::utils::source_zip::SourceZip, cache_size_limit: i32) /* thrown(java.lang.RuntimeException) */ -> javaparser::source_zip_type_solver::SourceZipTypeSolver {
		self.sources = sources;
		self.parse_if_necessary()?;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "SourceZipTypeSolver{" + "zipPath=" + self.sources.get_zip_path() + ", parent=" + self.parent + '}';
	}

	pub fn get_parent(&self) -> com::github::javaparser::resolution::type_solver::TypeSolver {
		return self.parent;
	}

	pub fn set_parent(&mut self, parent: &com::github::javaparser::resolution::type_solver::TypeSolver) /* thrown(java.lang.IllegalStateException) */ {
		Objects::requireNonNull(parent);
		if self.parent != null {
			return Err(IllegalStateException::new("This TypeSolver already has a parent."));
		}
		if parent == self {
			return Err(IllegalStateException::new("The parent of this TypeSolver cannot be itself."));
		}
		self.parent = parent;
	}

	pub fn try_to_solve_type(&self, name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::resolution::model::symbol_reference::SymbolReference {
		if !name.startsWith(self.common_prefix) {
			return SymbolReference::unsolved();
		}
	
		let td: TypeDeclaration<?> = self.types.get(name);
		if td == null {
			return SymbolReference::unsolved();
		}
	
		return SymbolReference.solved(&JavaParserFacade.get(self).getTypeDeclaration(td));
	}

	pub fn try_to_solve_type_in_module(&self, qualified_module_name: &/* Java */ java::lang::String /**/, simple_type_name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::resolution::model::symbol_reference::SymbolReference {
		return self.try_to_solve_type(qualified_module_name + "." + simple_type_name);
	}

	pub fn parse_if_necessary(&mut self) /* thrown(java.io.IOException | java.lang.RuntimeException) */ {
		if !self.types.isEmpty() {
			return;
		}
	
		let r0 = 'try0: {
			match self.sources.parse(|(path, result)|{
				self.paths.add(path);
				if result.get_result().isEmpty() {
					return;
				}
	
				for td in result.get_result().get().get_types() {
					self.accept(td);
				}
			}) {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			self.common_prefix = StringUtils::get_common_prefix(&self.types.keySet().toArray(: [Option<String>; 0] = [None; 0]));
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ IOException) => {
				break 'try0 Err(RuntimeException::new("Issue while parsing while type solving: " + self.sources.get_zip_path().toAbsolutePath(), e));
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	fn accept(&self, node: &com::github::javaparser::ast::node::Node) {
		if node instanceof TypeDeclaration<?> {
			self.types.put(&/* Java*/ td/* */ .get_fully_qualified_name().orElse(&/* Java*/ td/* */ .get_name_as_string()), /* Java*/ td/* */ );
		}
	
		node.get_child_nodes().forEach(self::accept);
	}
}

impl com::github::javaparser::resolution::type_solver::TypeSolver for SourceZipTypeSolver {}