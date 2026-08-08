use crate::com::github::javaparser::ParserConfiguration;
use java::nio::file::Path;
use java::util::ArrayList;
use java::util::List;
use java::util::Map;
use java::util::Optional;
use java::util::concurrent::ConcurrentHashMap;

pub struct ProjectRoot {
	root: /* Java */ java::nio::file::Path /**/,
	cache: /* Java */ java::util::Map /**/ = ConcurrentHashMap<>::new(),
	parser_configuration: com::github::javaparser::parser_configuration::ParserConfiguration,
}

impl ProjectRoot {
	pub fn new(root: &/* Java */ java::nio::file::Path /**/) -> com::github::javaparser::utils::project_root::ProjectRoot {
		this(root, ParserConfiguration::new());
	}

	pub fn new(root: &/* Java */ java::nio::file::Path /**/, parser_configuration: &com::github::javaparser::parser_configuration::ParserConfiguration) -> com::github::javaparser::utils::project_root::ProjectRoot {
		self.root = root;
		self.parserConfiguration = parser_configuration;
	}

	pub fn get_source_root(&self, source_root: &/* Java */ java::nio::file::Path /**/) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(&self.cache.get(source_root));
	}

	pub fn get_source_roots(&self) -> /* Java */ java::util::List /**/ {
		return ArrayList<>::new(&self.cache.values());
	}

	pub fn add_source_root(&self, path: &/* Java */ java::nio::file::Path /**/) {
		self.cache.put(path, &SourceRoot::new(path).set_parser_configuration(self.parser_configuration));
	}

	pub fn get_root(&self) -> /* Java */ java::nio::file::Path /**/ {
		return self.root;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "ProjectRoot at " + self.root + " with " + self.cache.values().toString();
	}
}