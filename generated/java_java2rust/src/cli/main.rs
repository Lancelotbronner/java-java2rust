use javaparser_core::com::github::javaparser::ParserConfiguration;
use javaparser_core::com::github::javaparser::StaticJavaParser;
use crate::java2rust::JavaTranspiler;
use crate::java2rust::RustJar;
use picocli::CommandLine;
use picocli::CommandLine::Option;
use picocli::CommandLine::Parameters;
use java::io::File;
use java::io::IOException;

pub struct Main {
	output: /* Java */ java::io::File /**/,
	sources: &[/* Java */ java::io::File /**/],
	maven: &[/* Java */ java::lang::String /**/],
	language_level: com::github::javaparser::parser_configuration::LanguageLevel = ParserConfiguration::com::github::javaparser::parser_configuration::LanguageLevel::JAVA_25,
}

impl Main {
	fn main(&self, args: &&[/* Java */ java::lang::String /**/]) {
		let exit_code: i32 = CommandLine::new(Main::new()).execute(args);
		System::exit(exit_code);
	}

	pub fn run(&self) /* thrown(java.lang.IllegalArgumentException) */ {
		let config: ParserConfiguration = ParserConfiguration::new();
		config.set_language_level(self.language_level);
		StaticJavaParser::set_configuration(config)?;
		let transpiler: JavaTranspiler = JavaTranspiler::new();
		config.setSymbolResolver(transpiler::solver);
		 {
			let i: i32 = 0;
			while i < self.sources.length {
				{
					let r0 = 'try0: {
						self.sources[i] = self.sources[i].getCanonicalFile();
						break 'try0 Ok(());
					};
					match r0 {
						Err(e @ IOException) => {
							System::err.printf("Failed to resolve sources: %s\n ", e);
						},
						Err(e) => Err(e)?,
						Ok => (),
					}
				}
				i += 1;
			 }
		 }
	
		let has_error: bool = false;
		for dep in self.maven {
			System::out.println("=> " + dep);
			let r1 = 'try1: {
				if let Err(e) = transpiler.add_maven_dependency(dep) {
					return Err(e);
				};
				break 'try1 Ok(());
			};
			match r1 {
				Err(e @ Exception) => {
					System::err.println(e);
					has_error = true;
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
		}
		if has_error {
			return;
		}
	
		for sources in self.sources {
			System::out.printf("=> %s\n", sources);
			transpiler.add_source_directory(sources);
		}
		System::out.printf("==> Processing %s Java files...\n", &transpiler.crates.size());
		transpiler.preanalyze();
		System::out.printf("==> Analyzing %s Java files...\n", &transpiler.number_of_tasks_to_analyze());
		transpiler.analyze();
		System::out.printf("==> Generating '%s' crates...\n", &transpiler.crates.size());
		for jar in transpiler.crates {
			System::out.printf("\t%s\n", jar.name);
			let r2 = 'try2: {
				if let Err(e) = jar.generate(&self.output.toPath()) {
					return Err(e);
				};
				break 'try2 Ok(());
			};
			match r2 {
				Err(e @ IOException) => {
					System::err.printf("\tFailed to write: %s\n", &e.getLocalizedMessage());
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
		}
		System::out.println("==> Done!");
	}
}

impl /* Java */ java::lang::Runnable /**/ for Main {}