use crate::com::github::javaparser::utils::CodeGenerationUtils;
use crate::com::github::javaparser::utils::Utils::assertNotNull;
use java::nio::file::FileVisitResult;
use crate::com::github::javaparser::JavaParser;
use crate::com::github::javaparser::ParseProblemException;
use crate::com::github::javaparser::ParseResult;
use crate::com::github::javaparser::ParserConfiguration;
use crate::com::github::javaparser::ast::CompilationUnit;
use crate::com::github::javaparser::printer::DefaultPrettyPrinter;
use java::io::IOException;
use java::nio::charset::Charset;
use java::nio::file::FileVisitResult;
use java::nio::file::Files;
use java::nio::file::Path;
use java::nio::file::SimpleFileVisitor;
use java::nio::file::attribute::BasicFileAttributes;
use java::util::ArrayList;
use java::util::List;
use java::util::Map;
use java::util::concurrent::ConcurrentHashMap;
use java::util::concurrent::ForkJoinPool;
use java::util::concurrent::RecursiveAction;
use java::util::function::Function;
use java::util::regex::Pattern;
use java::util::stream::Collectors;

pub struct SourceRoot {
	root: /* Java */ java::nio::file::Path /**/,
	cache: /* Java */ java::util::Map /**/ = ConcurrentHashMap<>::new(),
	parser_configuration: com::github::javaparser::parser_configuration::ParserConfiguration = ParserConfiguration::new(),
	printer: /* Java */ java::util::function::Function /**/ = DefaultPrettyPrinter::new()::print,
}

impl SourceRoot {
	static JAVA_IDENTIFIER: /* Java */ java::util::regex::Pattern /**/ = Pattern::compile("\\p{javaJavaIdentifierStart}\\p{javaJavaIdentifierPart}*");

	pub fn new(root: &/* Java */ java::nio::file::Path /**/) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::utils::source_root::SourceRoot {
		com::github::javaparser::utils::utils::Utils::assert_not_null(root)?;
		if !Files::isDirectory(root) {
			return Err(IllegalArgumentException::new("Only directories are allowed as root path: " + root));
		}
		self.root = root.normalize();
		Log::info("New source root at \"%s\"", |()|self.root);
	}

	pub fn new(root: &/* Java */ java::nio::file::Path /**/, parser_configuration: &com::github::javaparser::parser_configuration::ParserConfiguration) -> com::github::javaparser::utils::source_root::SourceRoot {
		this(root);
		self.set_parser_configuration(parser_configuration);
	}

	pub fn try_to_parse(&self, start_package: &/* Java */ java::lang::String /**/, filename: &/* Java */ java::lang::String /**/, configuration: &com::github::javaparser::parser_configuration::ParserConfiguration) /* thrown(java.lang.AssertionError | java.io.IOException) */ -> com::github::javaparser::parse_result::ParseResult {
		com::github::javaparser::utils::utils::Utils::assert_not_null(start_package)?;
		com::github::javaparser::utils::utils::Utils::assert_not_null(filename)?;
		/* final */ let relative_path: Path = com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::file_in_package_relative_path(start_package, filename);
		if self.cache.containsKey(relative_path) {
			Log::trace("Retrieving cached %s", |()|relative_path);
			return self.cache.get(relative_path);
		}
		/* final */ let path: Path = self.root.resolve(relative_path);
		Log::trace("Parsing %s", |()|path);
		/* final */ let result: ParseResult<CompilationUnit> = JavaParser::new(configuration).parse(path)?;
		self.cache.put(relative_path, result);
		return result;
	}

	pub fn try_to_parse(&self, start_package: &/* Java */ java::lang::String /**/, filename: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | java.io.IOException) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.try_to_parse(start_package, filename, self.parser_configuration)?;
	}

	pub fn try_to_parse(&self, start_package: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | java.io.IOException) */ -> /* Java */ java::util::List /**/ {
		com::github::javaparser::utils::utils::Utils::assert_not_null(start_package)?;
		self.log_package(start_package);
		/* final */ let path: Path = com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::package_absolute_path(self.root, start_package);
		Files::walkFileTree(path, SimpleFileVisitor<Path>::new() {
			pub fn visit_file(&self, file: &Path, attrs: &BasicFileAttributes) -> /* throws IOException*/ Result<FileVisitResult, Rc<Exception>> {
				if !attrs.isDirectory() && file.toString().endsWith(".java") {
					let relative: Path = self.root.relativize(&file.getParent());
					self.try_to_parse(&relative.toString(), &file.getFileName().toString())?;
				}
				return CONTINUE;
			}
	
			pub fn pre_visit_directory(&self, dir: &Path, attrs: &BasicFileAttributes) -> /* throws IOException*/ Result<FileVisitResult, Rc<Exception>> {
				return  if self.is_sensible_directory_to_enter(dir) { CONTINUE } else { SKIP_SUBTREE };
			}
	
		});
		return self.get_cache();
	}

	pub fn visit_file(&self, file: &/* Java */ java::nio::file::Path /**/, attrs: &/* Java */ java::nio::file::attribute::BasicFileAttributes /**/) /* thrown(java.lang.AssertionError | java.io.IOException) */ -> /* Java */ java::nio::file::FileVisitResult /**/ {
		if !attrs.isDirectory() && file.toString().endsWith(".java") {
			let relative: Path = self.root.relativize(&file.getParent());
			self.try_to_parse(&relative.toString(), &file.getFileName().toString())?;
		}
		return CONTINUE;
	}

	pub fn pre_visit_directory(&self, dir: &/* Java */ java::nio::file::Path /**/, attrs: &/* Java */ java::nio::file::attribute::BasicFileAttributes /**/) /* thrown(java.io.IOException) */ -> /* Java */ java::nio::file::FileVisitResult /**/ {
		return  if self.is_sensible_directory_to_enter(dir) { CONTINUE } else { SKIP_SUBTREE };
	}

	fn is_sensible_directory_to_enter(&self, dir: &/* Java */ java::nio::file::Path /**/) /* thrown(java.io.IOException) */ -> bool {
		/* final */ let dir_to_enter: String = dir.getFileName().toString();
		// Don't enter directories that cannot be packages.
		/* final */ let directory_is_a_valid_java_identifier: bool = self.JAVA_IDENTIFIER.matcher(dir_to_enter).matches();
		// But we can enter in root directory even if the root directory is not considered as a valid java identifier
		if !self.root.equals(dir) && (Files::isHidden(dir) || !directory_is_a_valid_java_identifier) {
			Log::trace("Not processing directory \"%s\"", |()|dir_to_enter);
			return false;
		}
		return true;
	}

	pub fn try_to_parse(&self) /* thrown(java.lang.AssertionError | java.io.IOException) */ -> /* Java */ java::util::List /**/ {
		return self.try_to_parse("")?;
	}

	pub fn try_to_parse_parallelized(&self, start_package: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> /* Java */ java::util::List /**/ {
		com::github::javaparser::utils::utils::Utils::assert_not_null(start_package)?;
		self.log_package(start_package);
		/* final */ let path: Path = com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::package_absolute_path(self.root, start_package);
		let parse: ParallelParse = ParallelParse::new(path, |(file, attrs)|{
			if !attrs.isDirectory() && file.toString().endsWith(".java") {
				let relative: Path = self.root.relativize(&file.getParent());
				let r0 = 'try0: {
					.tryToParse(&relative.toString(), &file.getFileName().toString(), self.parser_configuration);
					break 'try0 Ok(());
				};
				match r0 {
					Err(e @ IOException) => {
						Log::error(e);
					},
					Err(e) => Err(e)?,
					Ok => (),
				}
			}
			return CONTINUE;
		});
		let pool: ForkJoinPool = ForkJoinPool::new();
		pool.invoke(parse);
		return self.get_cache();
	}

	pub fn try_to_parse_parallelized(&self) /* thrown(java.lang.AssertionError) */ -> /* Java */ java::util::List /**/ {
		return self.try_to_parse_parallelized("")?;
	}

	pub fn parse(&self, start_package: &/* Java */ java::lang::String /**/, filename: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | com.github.javaparser.ParseProblemException) */ -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		com::github::javaparser::utils::utils::Utils::assert_not_null(start_package)?;
		com::github::javaparser::utils::utils::Utils::assert_not_null(filename)?;
		let r0 = 'try0: {
			/* final */ let result: ParseResult<CompilationUnit> = match self.try_to_parse(start_package, filename) {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			if result.is_successful() {
				return result.get_result().get();
			}
			break 'try0 Err(ParseProblemException::new(&result.get_problems()));
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ IOException) => {
				return Err(ParseProblemException::new(e));
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	fn callback(&self, absolute_path: &/* Java */ java::nio::file::Path /**/, configuration: &com::github::javaparser::parser_configuration::ParserConfiguration, callback: &com::github::javaparser::utils::source_root::Callback) /* thrown(java.lang.AssertionError | java.io.IOException) */ -> /* Java */ java::nio::file::FileVisitResult /**/ {
		let local_path: Path = self.root.relativize(absolute_path);
		Log::trace("Parsing %s", |()|local_path);
		let result: ParseResult<CompilationUnit> = JavaParser::new(configuration).parse(absolute_path)?;
		match callback.process(local_path, absolute_path, result) {
			SAVE => result.get_result().ifPresent(|cu|self.save(cu, absolute_path)),
			DONT_SAVE =>  {
				return CONTINUE;
			}
			TERMINATE =>  {
				return TERMINATE;
			}
			_ =>  {
				return Err(AssertionError::new("Return an enum defined in SourceRoot.Callback.Result"));
			}
		}
	}

	pub fn parse(&self, start_package: &/* Java */ java::lang::String /**/, filename: &/* Java */ java::lang::String /**/, configuration: &com::github::javaparser::parser_configuration::ParserConfiguration, callback: &com::github::javaparser::utils::source_root::Callback) /* thrown(java.lang.AssertionError | java.io.IOException) */ -> com::github::javaparser::utils::source_root::SourceRoot {
		com::github::javaparser::utils::utils::Utils::assert_not_null(start_package)?;
		com::github::javaparser::utils::utils::Utils::assert_not_null(filename)?;
		com::github::javaparser::utils::utils::Utils::assert_not_null(configuration)?;
		com::github::javaparser::utils::utils::Utils::assert_not_null(callback)?;
		self.callback(&com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::file_in_package_absolute_path(self.root, start_package, filename), configuration, callback)?;
		return self;
	}

	pub fn parse(&self, start_package: &/* Java */ java::lang::String /**/, filename: &/* Java */ java::lang::String /**/, callback: &com::github::javaparser::utils::source_root::Callback) /* thrown(java.lang.AssertionError | java.io.IOException) */ -> com::github::javaparser::utils::source_root::SourceRoot {
		self.parse(start_package, filename, self.parser_configuration, callback)?;
		return self;
	}

	pub fn parse(&self, start_package: &/* Java */ java::lang::String /**/, configuration: &com::github::javaparser::parser_configuration::ParserConfiguration, callback: &com::github::javaparser::utils::source_root::Callback) /* thrown(java.lang.AssertionError | java.io.IOException) */ -> com::github::javaparser::utils::source_root::SourceRoot {
		com::github::javaparser::utils::utils::Utils::assert_not_null(start_package)?;
		com::github::javaparser::utils::utils::Utils::assert_not_null(configuration)?;
		com::github::javaparser::utils::utils::Utils::assert_not_null(callback)?;
		self.log_package(start_package);
		/* final */ let path: Path = com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::package_absolute_path(self.root, start_package);
		if Files::exists(path) {
			Files::walkFileTree(path, SimpleFileVisitor<Path>::new() {
				pub fn visit_file(&self, absolute_path: &Path, attrs: &BasicFileAttributes) -> /* throws IOException*/ Result<FileVisitResult, Rc<Exception>> {
					if !attrs.isDirectory() && absolute_path.toString().endsWith(".java") {
						return self.callback(absolute_path, configuration, callback)?;
					}
					return CONTINUE;
				}
	
				pub fn pre_visit_directory(&self, dir: &Path, attrs: &BasicFileAttributes) -> /* throws IOException*/ Result<FileVisitResult, Rc<Exception>> {
					return  if self.is_sensible_directory_to_enter(dir)? { CONTINUE } else { SKIP_SUBTREE };
				}
	
			});
		}
		return self;
	}

	pub fn visit_file(&self, absolute_path: &/* Java */ java::nio::file::Path /**/, attrs: &/* Java */ java::nio::file::attribute::BasicFileAttributes /**/) /* thrown(java.lang.AssertionError | java.io.IOException) */ -> /* Java */ java::nio::file::FileVisitResult /**/ {
		if !attrs.isDirectory() && absolute_path.toString().endsWith(".java") {
			return self.callback(absolute_path, configuration, callback)?;
		}
		return CONTINUE;
	}

	pub fn pre_visit_directory(&self, dir: &/* Java */ java::nio::file::Path /**/, attrs: &/* Java */ java::nio::file::attribute::BasicFileAttributes /**/) /* thrown(java.io.IOException) */ -> /* Java */ java::nio::file::FileVisitResult /**/ {
		return  if self.is_sensible_directory_to_enter(dir)? { CONTINUE } else { SKIP_SUBTREE };
	}

	pub fn parse(&self, start_package: &/* Java */ java::lang::String /**/, callback: &com::github::javaparser::utils::source_root::Callback) /* thrown(java.lang.AssertionError | java.io.IOException) */ -> com::github::javaparser::utils::source_root::SourceRoot {
		self.parse(start_package, self.parser_configuration, callback)?;
		return self;
	}

	fn log_package(&self, start_package: &/* Java */ java::lang::String /**/) {
		if start_package.isEmpty() {
			return;
		}
		Log::info("Parsing package \"%s\"", |()|start_package);
	}

	pub fn parse_parallelized(&self, start_package: &/* Java */ java::lang::String /**/, configuration: &com::github::javaparser::parser_configuration::ParserConfiguration, callback: &com::github::javaparser::utils::source_root::Callback) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::utils::source_root::SourceRoot {
		com::github::javaparser::utils::utils::Utils::assert_not_null(start_package)?;
		com::github::javaparser::utils::utils::Utils::assert_not_null(configuration)?;
		com::github::javaparser::utils::utils::Utils::assert_not_null(callback)?;
		self.log_package(start_package);
		/* final */ let path: Path = com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::package_absolute_path(self.root, start_package);
		if Files::exists(path) {
			let parse: ParallelParse = ParallelParse::new(path, |(absolute_path, attrs)|{
				if !attrs.isDirectory() && absolute_path.toString().endsWith(".java") {
					let r0 = 'try0: {
						return .callback(absolute_path, configuration, callback);
						break 'try0 Ok(());
					};
					match r0 {
						Err(e @ IOException) => {
							Log::error(e);
						},
						Err(e) => Err(e)?,
						Ok => (),
					}
				}
				return CONTINUE;
			});
			let pool: ForkJoinPool = ForkJoinPool::new();
			pool.invoke(parse);
		}
		return self;
	}

	pub fn parse_parallelized(&self, start_package: &/* Java */ java::lang::String /**/, callback: &com::github::javaparser::utils::source_root::Callback) /* thrown(java.lang.AssertionError | java.io.IOException) */ -> com::github::javaparser::utils::source_root::SourceRoot {
		return self.parse_parallelized(start_package, self.parserConfiguration, callback)?;
	}

	pub fn parse_parallelized(&self, callback: &com::github::javaparser::utils::source_root::Callback) /* thrown(java.lang.AssertionError | java.io.IOException) */ -> com::github::javaparser::utils::source_root::SourceRoot {
		return self.parse_parallelized("", self.parserConfiguration, callback)?;
	}

	pub fn add(&self, start_package: &/* Java */ java::lang::String /**/, filename: &/* Java */ java::lang::String /**/, compilation_unit: &com::github::javaparser::ast::compilation_unit::CompilationUnit) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::utils::source_root::SourceRoot {
		com::github::javaparser::utils::utils::Utils::assert_not_null(start_package)?;
		com::github::javaparser::utils::utils::Utils::assert_not_null(filename)?;
		com::github::javaparser::utils::utils::Utils::assert_not_null(compilation_unit)?;
		Log::trace("Adding new file %s.%s", |()|start_package, |()|filename);
		/* final */ let path: Path = com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::file_in_package_relative_path(start_package, filename);
		/* final */ let parse_result: ParseResult<CompilationUnit> = ParseResult<>::new(compilation_unit, ArrayList<>::new(), null);
		self.cache.put(path, parse_result);
		return self;
	}

	pub fn add(&self, compilation_unit: &com::github::javaparser::ast::compilation_unit::CompilationUnit) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::utils::source_root::SourceRoot {
		com::github::javaparser::utils::utils::Utils::assert_not_null(compilation_unit)?;
		if compilation_unit.get_storage().isPresent() {
			/* final */ let path: Path = compilation_unit.get_storage().get().get_path();
			Log::trace("Adding new file %s", |()|path);
			/* final */ let parse_result: ParseResult<CompilationUnit> = ParseResult<>::new(compilation_unit, ArrayList<>::new(), null);
			self.cache.put(path, parse_result);
		} else {
			return Err(AssertionError::new("Files added with this method should have their path set."));
		}
		return self;
	}

	fn save(&self, cu: &com::github::javaparser::ast::compilation_unit::CompilationUnit, path: &/* Java */ java::nio::file::Path /**/) -> com::github::javaparser::utils::source_root::SourceRoot {
		return self.save(cu, path, &self.parser_configuration.get_character_encoding());
	}

	fn save(&self, cu: &com::github::javaparser::ast::compilation_unit::CompilationUnit, path: &/* Java */ java::nio::file::Path /**/, encoding: &/* Java */ java::nio::charset::Charset /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ -> com::github::javaparser::utils::source_root::SourceRoot {
		com::github::javaparser::utils::utils::Utils::assert_not_null(cu)?;
		com::github::javaparser::utils::utils::Utils::assert_not_null(path)?;
		cu.set_storage(path, encoding);
		cu.get_storage().get().save(self.printer)?;
		return self;
	}

	pub fn save_all(&self, root: &/* Java */ java::nio::file::Path /**/, encoding: &/* Java */ java::nio::charset::Charset /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::utils::source_root::SourceRoot {
		com::github::javaparser::utils::utils::Utils::assert_not_null(root)?;
		Log::info("Saving all files (%s) to %s", cache::size, |()|root);
		for e in self.cache.entrySet() {
			/* final */ let target: Path = self.resolve_path(root, &e.getKey())?;
			e.getValue().get_result().ifPresent(|cu|{
				Log::trace("Saving %s", |()|target);
				self.save(cu, target, encoding)?;
			});
		}
		return self;
	}

	fn resolve_path(&self, new_root: &/* Java */ java::nio::file::Path /**/, cached_path: &/* Java */ java::nio::file::Path /**/) /* thrown(java.lang.NullPointerException) */ -> /* Java */ java::nio::file::Path /**/ {
		if new_root == null || cached_path == null {
			return Err(NullPointerException::new("newRoot/cachedPath must not be null"));
		}
		return  if cached_path.isAbsolute() { cached_path.normalize() } else { new_root.resolve(cached_path).normalize() };
	}

	pub fn save_all(&self, root: &/* Java */ java::nio::file::Path /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::utils::source_root::SourceRoot {
		return self.save_all(root, &self.parser_configuration.get_character_encoding())?;
	}

	pub fn save_all(&self) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::utils::source_root::SourceRoot {
		return self.save_all(self.root)?;
	}

	pub fn save_all(&self, encoding: &/* Java */ java::nio::charset::Charset /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::utils::source_root::SourceRoot {
		return self.save_all(self.root, encoding)?;
	}

	pub fn get_cache(&self) -> /* Java */ java::util::List /**/ {
		return ArrayList<>::new(&self.cache.values());
	}

	pub fn get_compilation_units(&self) -> /* Java */ java::util::List /**/ {
		return self.cache.values().stream().filter(ParseResult::isSuccessful).map(|p|p.get_result().get()).collect(&Collectors::toList());
	}

	pub fn get_root(&self) -> /* Java */ java::nio::file::Path /**/ {
		return self.root;
	}

	pub fn get_parser_configuration(&self) -> com::github::javaparser::parser_configuration::ParserConfiguration {
		return self.parser_configuration;
	}

	pub fn set_parser_configuration(&mut self, parser_configuration: &com::github::javaparser::parser_configuration::ParserConfiguration) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::utils::source_root::SourceRoot {
		com::github::javaparser::utils::utils::Utils::assert_not_null(parser_configuration)?;
		self.parserConfiguration = parser_configuration;
		return self;
	}

	pub fn set_printer(&mut self, printer: &/* Java */ java::util::function::Function /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::utils::source_root::SourceRoot {
		com::github::javaparser::utils::utils::Utils::assert_not_null(printer)?;
		self.printer = printer;
		return self;
	}

	pub fn get_printer(&self) -> /* Java */ java::util::function::Function /**/ {
		return self.printer;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "SourceRoot at " + self.root;
	}
}

pub trait Callback;

enum Result;

struct ParallelParse {
	root: com::github::javaparser::utils::source_root::SourceRoot,
	callback: com::github::javaparser::utils::source_root::VisitFileCallback,
}

impl ParallelParse {
	static serialVersionUID: i64 = 1;

	fn new(path: &/* Java */ java::nio::file::Path /**/, callback: &com::github::javaparser::utils::source_root::VisitFileCallback) -> com::github::javaparser::utils::source_root::ParallelParse {
		self.root = SourceRoot::new(path);
		self.callback = callback;
	}

	fn compute(&self) {
		/* final */ let walks: List<ParallelParse> = ArrayList<>::new();
		let path: Path = self.root.get_root();
		let r0 = 'try0: {
			Files::walkFileTree(path, SimpleFileVisitor<Path>::new() {
				pub fn pre_visit_directory(&self, dir: &Path, attrs: &BasicFileAttributes) -> /* throws IOException*/ Result<FileVisitResult, Rc<Exception>> {
					if !match self.root.is_sensible_directory_to_enter(dir) {
						Err(e) => break 'try0 Err(e),
						Ok(s) => s,
					} {
						return SKIP_SUBTREE;
					}
					if !dir.equals(path) {
						let w: ParallelParse = ParallelParse::new(dir, self.callback);
						w.fork();
						walks.add(w);
						return SKIP_SUBTREE;
					}
					return CONTINUE;
				}
	
				pub fn visit_file(&self, file: &Path, attrs: &BasicFileAttributes) -> FileVisitResult {
					return self.callback.process(file, attrs);
				}
	
			});
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ IOException) => {
				Log::error(e);
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		for w in walks {
			w.join();
		}
	}

	pub fn pre_visit_directory(&self, dir: &/* Java */ java::nio::file::Path /**/, attrs: &/* Java */ java::nio::file::attribute::BasicFileAttributes /**/) /* thrown(java.io.IOException) */ -> /* Java */ java::nio::file::FileVisitResult /**/ {
		if !self.root.is_sensible_directory_to_enter(dir)? {
			return SKIP_SUBTREE;
		}
		if !dir.equals(path) {
			let w: ParallelParse = ParallelParse::new(dir, self.callback);
			w.fork();
			walks.add(w);
			return SKIP_SUBTREE;
		}
		return CONTINUE;
	}

	pub fn visit_file(&self, file: &/* Java */ java::nio::file::Path /**/, attrs: &/* Java */ java::nio::file::attribute::BasicFileAttributes /**/) -> /* Java */ java::nio::file::FileVisitResult /**/ {
		return self.callback.process(file, attrs);
	}
}

impl /* Java */ java::util::concurrent::Future /**/ for ParallelParse {}

impl /* Java */ java::io::Serializable /**/ for ParallelParse {}

trait VisitFileCallback;