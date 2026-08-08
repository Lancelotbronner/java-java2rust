use crate::com::github::javaparser::ParseStart::COMPILATION_UNIT;
use crate::com::github::javaparser::Providers::provider;
use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::JavaParser;
use crate::com::github::javaparser::ParseResult;
use crate::com::github::javaparser::ParserConfiguration;
use crate::com::github::javaparser::ast::CompilationUnit;
use java::io::IOException;
use java::nio::file::Path;
use java::nio::file::Paths;
use java::util::ArrayList;
use java::util::Collections;
use java::util::List;
use java::util::zip::ZipEntry;
use java::util::zip::ZipFile;

pub struct SourceZip {
	zip_path: /* Java */ java::nio::file::Path /**/,
	parser_configuration: com::github::javaparser::parser_configuration::ParserConfiguration,
}

impl SourceZip {
	pub fn new(zip_path: &/* Java */ java::nio::file::Path /**/) -> com::github::javaparser::utils::source_zip::SourceZip {
		this(zip_path, ParserConfiguration::new());
	}

	pub fn new(zip_path: &/* Java */ java::nio::file::Path /**/, configuration: &com::github::javaparser::parser_configuration::ParserConfiguration) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::utils::source_zip::SourceZip {
		com::github::javaparser::utils::utils::Utils::assert_not_null(zip_path)?;
		com::github::javaparser::utils::utils::Utils::assert_not_null(configuration)?;
		self.zipPath = zip_path.normalize();
		self.parserConfiguration = configuration;
		Log::info("New source zip at \"%s\"", |()|self.zipPath);
	}

	pub fn parse(&self) /* thrown(java.io.IOException) */ -> /* Java */ java::util::List /**/ {
		Log::info("Parsing zip at \"%s\"", |()|self.zip_path);
		let results: List<Pair<Path, ParseResult<CompilationUnit>>> = ArrayList<>::new();
		self.parse(|(path, result)|results.add(Pair<>::new(path, result)));
		return results;
	}

	pub fn parse(&self, callback: &com::github::javaparser::utils::source_zip::Callback) /* thrown(java.io.IOException) */ -> com::github::javaparser::utils::source_zip::SourceZip {
		Log::info("Parsing zip at \"%s\"", |()|self.zip_path);
		let java_parser: JavaParser = JavaParser::new(self.parser_configuration);
		let r0 = 'try0: {
			(let zip_file: ZipFile = ZipFile::new(&self.zip_path.toFile())) for entry in Collections::list(&zip_file.entries()) {
				if !entry.isDirectory() && entry.getName().endsWith(".java") {
					Log::info("Parsing zip entry \"%s\"", |()|entry.getName());
					/* final */ let result: ParseResult<CompilationUnit> = match java_parser.parse(, &match com::github::javaparser::providers::Providers::provider(&zip_file.getInputStream(entry)) {
						Err(e) => break 'try0 Err(e),
						Ok(s) => s,
					}) {
						Err(e) => break 'try0 Err(e),
						Ok(s) => s,
					};
					callback.process(&Paths::get(&entry.getName()), result);
				}
			}
			break 'try0 Ok(());
		};
		match r0 {
			Err(e) => Err(e)?,
			Ok => (),
		}
		return self;
	}

	pub fn get_zip_path(&self) -> /* Java */ java::nio::file::Path /**/ {
		return self.zip_path;
	}

	pub fn get_parser_configuration(&self) -> com::github::javaparser::parser_configuration::ParserConfiguration {
		return self.parser_configuration;
	}

	pub fn set_parser_configuration(&mut self, parser_configuration: &com::github::javaparser::parser_configuration::ParserConfiguration) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::utils::source_zip::SourceZip {
		com::github::javaparser::utils::utils::Utils::assert_not_null(parser_configuration)?;
		self.parserConfiguration = parser_configuration;
		return self;
	}
}

pub trait Callback;