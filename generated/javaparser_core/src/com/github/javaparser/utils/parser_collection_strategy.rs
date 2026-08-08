use java::nio::file::FileVisitResult;
use crate::com::github::javaparser::ParserConfiguration;
use java::io::IOException;
use java::nio::file;
use java::nio::file::attribute::BasicFileAttributes;

pub struct ParserCollectionStrategy {
	parser_configuration: com::github::javaparser::parser_configuration::ParserConfiguration,
	current_root: /* Java */ java::nio::file::Path /**/,
	java_matcher: /* Java */ java::nio::file::PathMatcher /**/ = self.get_path_matcher("glob:**.java"),
}

impl ParserCollectionStrategy {
	pub fn new() -> com::github::javaparser::utils::parser_collection_strategy::ParserCollectionStrategy {
		this(ParserConfiguration::new());
	}

	pub fn new(parser_configuration: &com::github::javaparser::parser_configuration::ParserConfiguration) -> com::github::javaparser::utils::parser_collection_strategy::ParserCollectionStrategy {
		self.parserConfiguration = parser_configuration;
	}

	pub fn get_parser_configuration(&self) -> com::github::javaparser::parser_configuration::ParserConfiguration {
		return self.parser_configuration;
	}

	pub fn collect(&self, path: &/* Java */ java::nio::file::Path /**/) -> com::github::javaparser::utils::project_root::ProjectRoot {
		let project_root: ProjectRoot = ProjectRoot::new(path, self.parser_configuration);
		let r0 = 'try0: {
			Files::walkFileTree(path, SimpleFileVisitor<Path>::new() {
				let current_root: Path,
				/* final */ let java_matcher: PathMatcher = self.get_path_matcher("glob:**.java"),
				pub fn visit_file(&self, file: &Path, attrs: &BasicFileAttributes) -> FileVisitResult {
					if "module-info.java".equals(&file.getFileName().toString()) {
						// directory.
						return CONTINUE;
					}
					if self.java_matcher.matches(file) {
						self.current_root = self.get_root(file).orElse(null);
						if self.current_root != null {
							return SKIP_SIBLINGS;
						}
					}
					return CONTINUE;
				}
	
				pub fn pre_visit_directory(&self, dir: &Path, attrs: &BasicFileAttributes) -> /* throws IOException*/ Result<FileVisitResult, Rc<Exception>> {
					if Files::isHidden(dir) || (self.current_root != null && dir.startsWith(self.current_root)) {
						return SKIP_SUBTREE;
					}
					return CONTINUE;
				}
	
				pub fn post_visit_directory(&self, dir: &Path, e: &IOException) -> /* throws IOException*/ Result<FileVisitResult, Rc<Exception>> {
					if self.current_root != null && Files::isSameFile(dir, self.current_root) {
						project_root.add_source_root(dir);
						self.current_root = null;
					}
					return CONTINUE;
				}
	
			});
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ IOException) => {
				Log::error(e, "Unable to walk %s", |()|path);
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		return project_root;
	}

	pub fn visit_file(&mut self, file: &/* Java */ java::nio::file::Path /**/, attrs: &/* Java */ java::nio::file::attribute::BasicFileAttributes /**/) -> /* Java */ java::nio::file::FileVisitResult /**/ {
		if "module-info.java".equals(&file.getFileName().toString()) {
			// directory.
			return CONTINUE;
		}
		if self.java_matcher.matches(file) {
			self.current_root = self.get_root(file).orElse(null);
			if self.current_root != null {
				return SKIP_SIBLINGS;
			}
		}
		return CONTINUE;
	}

	pub fn pre_visit_directory(&self, dir: &/* Java */ java::nio::file::Path /**/, attrs: &/* Java */ java::nio::file::attribute::BasicFileAttributes /**/) /* thrown(java.io.IOException) */ -> /* Java */ java::nio::file::FileVisitResult /**/ {
		if Files::isHidden(dir) || (self.current_root != null && dir.startsWith(self.current_root)) {
			return SKIP_SUBTREE;
		}
		return CONTINUE;
	}

	pub fn post_visit_directory(&mut self, dir: &/* Java */ java::nio::file::Path /**/, e: &/* Java */ java::io::IOException /**/) /* thrown(java.io.IOException) */ -> /* Java */ java::nio::file::FileVisitResult /**/ {
		if self.current_root != null && Files::isSameFile(dir, self.current_root) {
			project_root.add_source_root(dir);
			self.current_root = null;
		}
		return CONTINUE;
	}
}

impl com::github::javaparser::utils::collection_strategy::CollectionStrategy for ParserCollectionStrategy {}