use crate::com::github::javaparser::ParseStart;
use crate::com::github::javaparser::Problem::PROBLEM_BY_BEGIN_POSITION;
use crate::com::github::javaparser::Providers::provider;
use crate::com::github::javaparser::Providers::resourceProvider;
use crate::com::github::javaparser::utils::Utils::assertNotNull;
use java::util::stream::Collectors::toList;
use crate::com::github::javaparser::ast::CompilationUnit;
use crate::com::github::javaparser::ast::ImportDeclaration;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::PackageDeclaration;
use crate::com::github::javaparser::ast::body::BodyDeclaration;
use crate::com::github::javaparser::ast::body::MethodDeclaration;
use crate::com::github::javaparser::ast::body::Parameter;
use crate::com::github::javaparser::ast::body::TypeDeclaration;
use crate::com::github::javaparser::ast::expr;
use crate::com::github::javaparser::ast::modules::ModuleDeclaration;
use crate::com::github::javaparser::ast::modules::ModuleDirective;
use crate::com::github::javaparser::ast::stmt::BlockStmt;
use crate::com::github::javaparser::ast::stmt::ExplicitConstructorInvocationStmt;
use crate::com::github::javaparser::ast::stmt::Statement;
use crate::com::github::javaparser::ast::type::ClassOrInterfaceType;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::ast::type::TypeParameter;
use java::io;
use java::nio::charset::Charset;
use java::nio::file::Path;
use java::util::List;
use java::util::function::Supplier;

pub struct JavaParser {
	configuration: com::github::javaparser::parser_configuration::ParserConfiguration,
	ast_parser: com::github::javaparser::generated_java_parser::GeneratedJavaParser = null,
}

impl JavaParser {
	pub fn new() -> com::github::javaparser::java_parser::JavaParser {
		this(ParserConfiguration::new());
	}

	pub fn new(configuration: &com::github::javaparser::parser_configuration::ParserConfiguration) -> com::github::javaparser::java_parser::JavaParser {
		self.configuration = configuration;
	}

	pub fn get_parser_configuration(&self) -> com::github::javaparser::parser_configuration::ParserConfiguration {
		return self.configuration;
	}

	fn get_parser_for_provider(&mut self, provider: &com::github::javaparser::provider::Provider) -> com::github::javaparser::generated_java_parser::GeneratedJavaParser {
		if self.ast_parser == null {
			self.ast_parser = GeneratedJavaParser::new(provider);
		} else {
			self.ast_parser.reset(provider);
		}
		self.ast_parser.set_tab_size(&self.configuration.get_tab_size());
		self.ast_parser.set_store_tokens(&self.configuration.is_store_tokens());
		let language_level: ParserConfiguration.LanguageLevel = self.configuration.get_language_level();
		if language_level != null {
			if language_level.is_yield_supported() {
				self.ast_parser.set_yield_supported();
			}
		}
		return self.ast_parser;
	}

	pub fn parse<N: com::github::javaparser::ast::node::Node>(&self, start: &com::github::javaparser::parse_start::ParseStart, mut provider: &com::github::javaparser::provider::Provider) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		com::github::javaparser::utils::utils::Utils::assert_not_null(start)?;
		com::github::javaparser::utils::utils::Utils::assert_not_null(provider)?;
		let processors: List<Processor> = self.configuration.get_processors().stream().map(Supplier::get).collect(&/* Java */ java::util::stream::Collectors /**/::toList());
		for processor in processors {
			provider = processor.pre_process(provider);
		}
		/* final */ let parser: GeneratedJavaParser = self.get_parser_for_provider(provider);
		let r0 = 'try0: {
			let result_node: N = start.parse(parser);
			let result: ParseResult<N> = ParseResult<>::new(result_node, parser.problems, &parser.get_comments_collection());
			for processor in processors {
				processor.post_process(result, self.configuration);
			}
			result.get_problems().sort();
			return result;
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ Exception) => {
				/* final */ let message: String =  if e.getMessage() == null { "Unknown error" } else { e.getMessage() };
				parser.problems.add(Problem::new(message, null, e));
				return ParseResult<>::new(null, parser.problems, &parser.get_comments_collection());
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		let r1 = 'try1: {
			provider.close();
			break 'try1 Ok(());
		};
		match r1 {
			Err(e @ IOException) => {
			// Since we're done parsing and have our result, we don't care about any errors.
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	
	}

	pub fn parse(&self, in: &/* Java */ java::io::InputStream /**/, encoding: &/* Java */ java::nio::charset::Charset /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::provider(in, encoding)?)?;
	}

	pub fn parse(&self, in: &/* Java */ java::io::InputStream /**/) /* thrown(java.lang.AssertionError | java.lang.RuntimeException) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(in, &self.configuration.get_character_encoding())?;
	}

	pub fn parse(&self, file: &/* Java */ java::io::File /**/, encoding: &/* Java */ java::nio::charset::Charset /**/) /* thrown(java.io.FileNotFoundException) */ -> com::github::javaparser::parse_result::ParseResult {
		let result: ParseResult<CompilationUnit> = self.parse(, &com::github::javaparser::providers::Providers::provider(file, encoding))?;
		result.get_result().ifPresent(|cu|cu.set_storage(&file.toPath(), encoding));
		return result;
	}

	pub fn parse(&self, file: &/* Java */ java::io::File /**/) /* thrown(java.io.FileNotFoundException) */ -> com::github::javaparser::parse_result::ParseResult {
		let result: ParseResult<CompilationUnit> = self.parse(, &com::github::javaparser::providers::Providers::provider(file, &self.configuration.get_character_encoding()))?;
		result.get_result().ifPresent(|cu|cu.set_storage(&file.toPath(), &self.configuration.get_character_encoding()));
		return result;
	}

	pub fn parse(&self, path: &/* Java */ java::nio::file::Path /**/, encoding: &/* Java */ java::nio::charset::Charset /**/) /* thrown(java.io.IOException) */ -> com::github::javaparser::parse_result::ParseResult {
		let result: ParseResult<CompilationUnit> = self.parse(, &com::github::javaparser::providers::Providers::provider(path, encoding))?;
		result.get_result().ifPresent(|cu|cu.set_storage(path, encoding));
		return result;
	}

	pub fn parse(&self, path: &/* Java */ java::nio::file::Path /**/) /* thrown(java.io.IOException) */ -> com::github::javaparser::parse_result::ParseResult {
		let result: ParseResult<CompilationUnit> = self.parse(, &com::github::javaparser::providers::Providers::provider(path, &self.configuration.get_character_encoding()))?;
		result.set_source_path(path);
		result.get_result().ifPresent(|cu|cu.set_storage(path, &self.configuration.get_character_encoding()));
		return result;
	}

	pub fn parse_resource(&self, path: &/* Java */ java::lang::String /**/) /* thrown(java.io.IOException | java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::resource_provider(path, &self.configuration.get_character_encoding()))?;
	}

	pub fn parse_resource(&self, path: &/* Java */ java::lang::String /**/, encoding: &/* Java */ java::nio::charset::Charset /**/) /* thrown(java.io.IOException | java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::resource_provider(path, encoding))?;
	}

	pub fn parse_resource(&self, class_loader: &/* Java */ java::lang::ClassLoader /**/, path: &/* Java */ java::lang::String /**/, encoding: &/* Java */ java::nio::charset::Charset /**/) /* thrown(java.io.IOException | java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::resource_provider(class_loader, path, encoding)?)?;
	}

	pub fn parse(&self, reader: &/* Java */ java::io::Reader /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::provider(reader))?;
	}

	pub fn parse(&self, code: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::provider(code))?;
	}

	pub fn parse_block(&self, block_statement: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::provider(block_statement))?;
	}

	pub fn parse_statement(&self, statement: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::provider(statement))?;
	}

	pub fn parse_import(&self, import_declaration: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::provider(import_declaration))?;
	}

	pub fn parse_expression<T: com::github::javaparser::ast::expr::expression::Expression>(&self, expression: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::provider(expression))? as ParseResult<T>;
	}

	pub fn parse_annotation(&self, annotation: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::provider(annotation))?;
	}

	pub fn parse_annotation_body_declaration(&self, body: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::provider(body))?;
	}

	pub fn parse_body_declaration<T: com::github::javaparser::ast::body::body_declaration::BodyDeclaration>(&self, body: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::provider(body))? as ParseResult<T>;
	}

	pub fn parse_class_or_interface_type(&self, type: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::provider(type))?;
	}

	pub fn parse_type(&self, type: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::provider(type))?;
	}

	pub fn parse_variable_declaration_expr(&self, declaration: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::provider(declaration))?;
	}

	pub fn parse_explicit_constructor_invocation_stmt(&self, statement: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::provider(statement))?;
	}

	pub fn parse_name(&self, qualified_name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::provider(qualified_name))?;
	}

	pub fn parse_simple_name(&self, name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::provider(name))?;
	}

	pub fn parse_parameter(&self, parameter: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::provider(parameter))?;
	}

	pub fn parse_package_declaration(&self, package_declaration: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::provider(package_declaration))?;
	}

	pub fn parse_type_declaration(&self, type_declaration: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::provider(type_declaration))?;
	}

	pub fn parse_module_declaration(&self, module_declaration: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::provider(module_declaration))?;
	}

	pub fn parse_module_directive(&self, module_directive: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::provider(module_directive))?;
	}

	pub fn parse_type_parameter(&self, type_parameter: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::provider(type_parameter))?;
	}

	pub fn parse_method_declaration(&self, method_declaration: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::provider(method_declaration))?;
	}

	pub fn parse_array_initializer_expr(&self, array_initializer_expr: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::parse_result::ParseResult {
		return self.parse(, &com::github::javaparser::providers::Providers::provider(array_initializer_expr))?;
	}
}