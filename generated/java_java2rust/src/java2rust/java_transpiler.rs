use javaparser_core::com::github::javaparser::StaticJavaParser;
use javaparser_core::com::github::javaparser::ast::body::MethodDeclaration;
use javaparser_core::com::github::javaparser::ast::expr::Expression;
use javaparser_core::com::github::javaparser::ast::expr::Name;
use javaparser_core::com::github::javaparser::ast::stmt::Statement;
use javaparser_core::com::github::javaparser::ast::type::Type;
use javaparser_core::com::github::javaparser::quality::NotNull;
use javaparser_core::com::github::javaparser::resolution::declarations::ResolvedMethodDeclaration;
use javaparser_core::com::github::javaparser::resolution::declarations::ResolvedReferenceTypeDeclaration;
use javaparser_core::com::github::javaparser::resolution::types::ResolvedType;
use javaparser_core::com::github::javaparser::symbolsolver::JavaSymbolSolver;
use javaparser_core::com::github::javaparser::symbolsolver::resolution::typesolvers::CombinedTypeSolver;
use javaparser_core::com::github::javaparser::symbolsolver::resolution::typesolvers::JarTypeSolver;
use javaparser_core::com::github::javaparser::symbolsolver::resolution::typesolvers::ReflectionTypeSolver;
use javaparser_core::com::github::javaparser::utils::SourceZip;
use crate::java2rust::rust;
use crate::javaparser::SourceZipTypeSolver;
use commons_lang3::org::apache::commons::io::FilenameUtils;
use commons_lang3::org::jspecify::annotations::NonNull;
use commons_lang3::org::jspecify::annotations::Nullable;
use java::io::File;
use java::io::IOException;
use java::nio::file::Files;
use java::nio::file::Path;
use java::nio::file::Paths;
use java::util;
use java::util::function::Supplier;
use java::util::function::UnaryOperator;

pub struct JavaTranspiler {
	crates: /* Java */ java::util::List /**/ = ArrayList<>::new(),
	solver: /* Unsolved symbol : JavaSymbolSolver */ JavaSymbolSolver,
	external_type_solver: java2rust::transpiler_type_solver::TranspilerTypeSolver,
	solvers: /* Unsolved symbol : CombinedTypeSolver */ CombinedTypeSolver = CombinedTypeSolver::new(),
	directories: /* Java */ java::util::Set /**/ = HashSet<>::new(),
	names: /* Java */ java::util::HashMap /**/ = HashMap<>::new(),
	errors: /* Java */ java::util::Set /**/ = HashSet<>::new(),
	methods: /* Java */ java::util::Map /**/ = HashMap<>::new(),
}

impl JavaTranspiler {
	pub fn new() -> java2rust::java_transpiler::JavaTranspiler {
		self::solver = JavaSymbolSolver::new(self.solvers);
		self.solvers.setExceptionHandler(CombinedTypeSolver.ExceptionHandlers.IGNORE_ALL);
		self.solvers.add(ReflectionTypeSolver::new());
		self.external_type_solver = TranspilerTypeSolver::new(self);
		self.solvers.add(self.external_type_solver);
	}

	pub fn locate(&self, mut name: &com::github::javaparser::ast::expr::name::Name) -> java2rust::rust::rust_package::RustPackage {
		while true {
			for crate in self.crates {
				let path: String = name.as_string();
				let pkg: RustPackage = crate.lib.locate(path);
				if pkg != null {
					return pkg;
				}
	
			}
			let qualifier: Optional<Name> = name.get_qualifier();
			if qualifier.isEmpty() {
				return null;
			}
	
			name = qualifier.get();
		}
	}

	pub fn add_maven_dependency(&self, maven: &/* Java */ java::lang::String /**/) /* thrown(java.lang.Exception) */ {
		let components: Vec<String> = maven.split(":", 4);
		if components.length != 3 {
			return Err(Exception::new("Invalid maven dependency string '" + maven + "'"));
		}
	
		let path: Vec<String> = components[0].split("\\.");
		let resolved: Path = Paths::get(&System::getProperty("user.home"), ".m2").resolve("repository", path).resolve(components[1], components[2]);
		let sources: Path = resolved.resolve(components[1] + "-" + components[2] + "-sources.jar").toRealPath();
		if Files::exists(sources) {
			self.add_source_zip(maven, components[1], sources);
			return;
		}
		System::out.println("\tCould not locate sources jar");
		let jar: Path = resolved.resolve(components[1] + "-" + components[2] + ".jar").toRealPath();
		if !Files::exists(jar) {
			return Err(Exception::new(&"Could not locate '%s' at '%s'".formatted(maven, jar)));
		}
	
		System::out.println("\tRegistered jar");
		self.solvers.add(JarTypeSolver::new(jar));
	}

	pub fn add_source_zip(&self, id: &/* Java */ java::lang::String /**/, name: &/* Java */ java::lang::String /**/, path: &/* Java */ java::nio::file::Path /**/) /* thrown(java.io.IOException) */ {
		let zip: SourceZip = SourceZip::new(path, &StaticJavaParser::get_parser_configuration());
		let solver: SourceZipTypeSolver = SourceZipTypeSolver::new(zip);
		self.solvers.add(solver);
		System::out.println("\tParsing sources jar...");
		solver.parse_if_necessary()?;
		for file in solver.paths {
			System::out.printf("\t%s\n", file);
		}
		System::out.printf("\tParsed %s source files%n", &solver.paths.size());
		System::out.printf("\tRegistered %s types%n", &solver.types.size());
		let jar: RustJar = RustJar::sources(id, name, zip)?;
		self.add_jar(jar);
	}

	pub fn add_jar(&self, jar: &java2rust::rust::rust_jar::RustJar) {
		//TODO: ensure preliminary visits are made?
		self.crates.add(jar);
	}

	pub fn add_source_directory(&self, input: &/* Java */ java::io::File /**/) {
		let src: File = input.toPath().resolve("src").toFile();
		let jar: RustJar = RustJar::new(&input.getAbsolutePath(), &FilenameUtils.removeExtension(&input.getName()), &src.toPath());
		self.add_jar(jar);
		if self.directories.contains(src) {
			return;
		}
	
		if !self.add_item(src, src, jar, jar.lib) {
			return;
		}
	
		System::out.printf("\tParsed %s source files%n", &jar.units.size());
	}

	pub fn add_source_code(&self, filename: &/* Java */ java::lang::String /**/, code: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ {
		if self.crates.isEmpty() {
			self.crates.add(RustJar::new("test", "test", &Path::of("test")));
		}
		let jar: RustJar = self.crates.getFirst();
		jar.add_source_code(&Path::of(filename), jar.lib, code)?;
	}

	pub fn add_script(&self, filename: &/* Java */ java::lang::String /**/, code: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ {
		if self.crates.isEmpty() {
			self.crates.add(RustJar::new("test", "test", &Path::of("test")));
		}
	
		let jar: RustJar = self.crates.getFirst();
		jar.add_source_code(&Path::of(filename), jar.lib, code)?;
	}

	pub fn register(&self, method: &java2rust::rust::rust_method::RustMethod) {
		self.methods.put(&method.resolved.get_qualified_signature(), method);
	}

	pub fn method(&self, qualified_signature: &/* Java */ java::lang::String /**/) -> java2rust::rust::rust_method::RustMethod {
		let m: RustMethod = self.methods.get(qualified_signature);
		if m == null && self.errors.add(qualified_signature) {
			System::err.printf(&"Unknown method (not registered) '%s'\n".formatted(qualified_signature));
		}
	
		return m;
	}

	pub fn method(&self, resolved: &com::github::javaparser::resolution::declarations::resolved_method_declaration::ResolvedMethodDeclaration) -> java2rust::rust::rust_method::RustMethod {
		return self.methods.get(&resolved.get_qualified_signature());
	}

	pub fn method(&self, decl: &com::github::javaparser::ast::body::method_declaration::MethodDeclaration) -> java2rust::rust::rust_method::RustMethod {
		let resolved: ResolvedMethodDeclaration = decl.resolve();
		let signature: String = resolved.get_qualified_signature();
		let method: RustMethod = self.methods.get(signature);
		if method == null {
			method = RustMethod::new(null, decl, resolved);
			self.methods.put(signature, method);
		}
		return method;
	}

	pub fn number_of_tasks_to_analyze(&self) -> i64 {
		return self.crates.size();
	}

	pub fn preanalyze(&self) {
		self.external_type_solver.reload();
		//TODO: Also build id-to-module cache? for import resolution
		for jar in self.crates {
			jar.preanalyze(self);
		}
	}

	pub fn analyze(&self) {
		for jar in self.crates {
			jar.analyze(self);
		}
	//TODO: infer dependencies between crates?
	}

	pub fn generate(&self, output: &/* Java */ java::nio::file::Path /**/) /* thrown(java.io.IOException) */ {
		for jar in self.crates {
			jar.generate(output)?;
		}
	}

	pub fn register(&self, item: &java2rust::rust::rust_item::RustItem) {
		self.names.put(&item.id(), &item.path());
	}

	pub fn register_name(&self, id: &/* Java */ java::lang::String /**/, name: &/* Java */ java::lang::String /**/) {
		self.names.put(id, name);
	}

	pub fn name_of(&self, id: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return self.names.get(id);
	}

	pub fn describe(&self, type: &com::github::javaparser::ast::type::type::Type) /* thrown(java.lang.UnsupportedOperationException) */ -> /* Java */ java::lang::String /**/ {
		if type.is_void_type() {
			return "()";
		}
	
		let r0 = 'try0: {
			let ty: ResolvedType = type.resolve();
			if ty != null {
				return match self.describe(ty) {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				};
			}
	
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ Throwable) => {
				System::err.printf("Couldn't describe type: %s\n", &e.getLocalizedMessage());
				return "/* %s */ %s".formatted(&e.getMessage(), type);
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		return "/* Java */ %s".formatted(type);
	}

	pub fn describe(&self, ty: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) /* thrown(java.lang.UnsupportedOperationException) */ -> /* Java */ java::lang::String /**/ {
		if ty.is_primitive() {
			return match ty.as_primitive()? {
			BYTE => "i8",
			SHORT => "i16",
			CHAR => "u16",
			INT => "i32",
			LONG => "i64",
			BOOLEAN => "bool",
			FLOAT => "f32",
			DOUBLE => "f64",
			};
		}
	
		if ty.is_array() {
			return "&[%s]".formatted(&self.describe(&ty.as_array_type()?.get_component_type())?);
		}
	
		if ty.is_reference_type() {
			return self.describe_via_id(&ty.as_reference_type()?.get_id(), &ty.as_reference_type()?.describe());
		}
	
		if ty.is_type_variable() {
			return ty.as_type_variable()?.describe();
		}
	
		return Err(UnsupportedOperationException::new("Unknown ResolvedType " + ty));
	}

	pub fn describe(&self, ty: &com::github::javaparser::resolution::declarations::resolved_reference_type_declaration::ResolvedReferenceTypeDeclaration) -> /* Java */ java::lang::String /**/ {
		return self.describe_via_id(&ty.get_id(), &ty.get_name());
	}

	fn describe_via_id(&self, id: &/* Java */ java::lang::String /**/, insert: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return self.describe_via_id(id, |()|insert);
	}

	fn describe_via_id(&self, id: &/* Java */ java::lang::String /**/, insert: &/* Java */ java::util::function::Supplier /**/) -> /* Java */ java::lang::String /**/ {
		if self.names.get(id) instanceof String {
			return /* Java*/ name/* */ ;
		}
	
		if self.errors.add(id) {
			System::err.printf("Unknown identifier (not related to type solving) '%s'%n", id);
		}
	
		//		return names.put(id, name);
		return "/* Java */ %s /**/".formatted(&id.replace(".", "::"));
	}

	fn describe_via_id(&self, id: &/* Java */ java::lang::String /**/, insert: &/* Java */ java::lang::String /**/, convert: &/* Java */ java::util::function::UnaryOperator /**/) -> /* Java */ java::lang::String /**/ {
		return self.describe_via_id(id, |()|convert.apply(insert));
	}

	pub fn name_of(&self, id: &/* Java */ java::lang::String /**/, default_value: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		let name: String = self.names.get(id);
		return  if name == null { default_value } else { name };
	}

	pub fn describe(&self, expr: &com::github::javaparser::ast::expr::expression::Expression) -> /* Java */ java::lang::String /**/ {
		if expr == null {
			return "";
		}
	
		let visitor: RustVisitor = RustVisitor::new(self);
		expr.accept(visitor, null);
		return visitor.to_string();
	}

	pub fn describe(&self, stmt: &com::github::javaparser::ast::stmt::statement::Statement, method: &java2rust::rust::i_rust_function::IRustFunction) -> /* Java */ java::lang::String /**/ {
		if stmt == null {
			return "";
		}
	
		let visitor: RustVisitor = RustVisitor::new(self);
		visitor.method = method;
		if method != null {
			visitor.item = method.item();
		}
	
		stmt.accept(visitor, null);
		return visitor.to_string();
	}

	fn add_sub_item(&self, input: &/* Java */ java::io::File /**/, root: &/* Java */ java::io::File /**/, jar: &java2rust::rust::rust_jar::RustJar, parent_module: &java2rust::rust::rust_package::RustPackage) -> bool {
		let module_name: String = FilenameUtils.removeExtension(&input.getName());
		let module: RustPackage = parent_module.submodule(module_name, RustVisibility::java2rust::rust::rust_visibility::RustVisibility::PUB);
		let added: bool = self.add_item(input, root, jar, module);
		if !added {
			module.delete();
		}
	
		return added;
	}

	fn add_item(&self, input: &/* Java */ java::io::File /**/, root: &/* Java */ java::io::File /**/, jar: &java2rust::rust::rust_jar::RustJar, module: &java2rust::rust::rust_package::RustPackage) /* thrown(java.io.IOException) */ -> bool {
		let children: Vec<File> = input.listFiles();
		if children == null {
			if input.getPath().endsWith(".java") {
				let r0 = 'try0: {
					if let Err(e) = jar.add(&input.toPath(), module) {
						return Err(e);
					};
					break 'try0 Ok(());
				};
				match r0 {
					Err(e @ IOException) => {
						System::err.printf("Couldn't add unit to jar: %s\n", e);
					},
					Err(e) => Err(e)?,
					Ok => (),
				}
				return true;
			}
			return false;
		}
		let contains_source: bool = false;
		for file in children {
			if self.add_sub_item(file, root, jar, module) {
				contains_source = true;
			}
	
		}
		if contains_source {
			self.directories.add(input);
		}
	
		return true;
	}
}