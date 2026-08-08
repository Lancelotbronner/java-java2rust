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
use crate::com::github::javaparser::javadoc::Javadoc;
use crate::com::github::javaparser::quality::NotNull;
use crate::com::github::javaparser::quality::Preconditions;
use java::io;
use java::nio::charset::Charset;
use java::nio::file::Path;

pub struct StaticJavaParser;

impl StaticJavaParser {
	static localConfiguration: /* Java */ java::lang::ThreadLocal /**/ = ThreadLocal::withInitial(ParserConfiguration::new);

	pub fn get_configuration(&self) -> com::github::javaparser::parser_configuration::ParserConfiguration {
		return com::github::javaparser::static_java_parser::StaticJavaParser::get_parser_configuration();
	}

	pub fn get_parser_configuration(&self) -> com::github::javaparser::parser_configuration::ParserConfiguration {
		return self.local_configuration.get();
	}

	pub fn set_configuration(&self, configuration: &com::github::javaparser::parser_configuration::ParserConfiguration) /* thrown(java.lang.IllegalArgumentException) */ {
		Preconditions::check_not_null(configuration, "Parameter configuration can't be null.")?;
		self.local_configuration.set(configuration);
	}

	pub fn parse(&self, in: &/* Java */ java::io::InputStream /**/, encoding: &/* Java */ java::nio::charset::Charset /**/) /* thrown(java.lang.AssertionError | com.github.javaparser.ParseProblemException | java.lang.RuntimeException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		Preconditions::check_not_null(in, "Parameter in can't be null.")?;
		Preconditions::check_not_null(encoding, "Parameter encoding can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::handle_result(&com::github::javaparser::static_java_parser::StaticJavaParser::new_parser().parse(in, encoding)?)?;
	}

	pub fn parse(&self, in: &/* Java */ java::io::InputStream /**/) /* thrown(java.lang.AssertionError | com.github.javaparser.ParseProblemException | java.lang.RuntimeException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		Preconditions::check_not_null(in, "Parameter in can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse(in)?;
	}

	pub fn parse(&self, file: &/* Java */ java::io::File /**/, encoding: &/* Java */ java::nio::charset::Charset /**/) /* thrown(java.io.FileNotFoundException | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		Preconditions::check_not_null(file, "Parameter file can't be null.")?;
		Preconditions::check_not_null(encoding, "Parameter encoding can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::handle_result(&com::github::javaparser::static_java_parser::StaticJavaParser::new_parser().parse(file, encoding)?)?;
	}

	pub fn parse(&self, file: &/* Java */ java::io::File /**/) /* thrown(java.io.FileNotFoundException | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		Preconditions::check_not_null(file, "Parameter file can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse(file)?;
	}

	pub fn parse(&self, path: &/* Java */ java::nio::file::Path /**/, encoding: &/* Java */ java::nio::charset::Charset /**/) /* thrown(java.io.IOException | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		Preconditions::check_not_null(path, "Parameter path can't be null.")?;
		Preconditions::check_not_null(encoding, "Parameter encoding can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::handle_result(&com::github::javaparser::static_java_parser::StaticJavaParser::new_parser().parse(path, encoding)?)?;
	}

	pub fn parse(&self, path: &/* Java */ java::nio::file::Path /**/) /* thrown(java.io.IOException | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		Preconditions::check_not_null(path, "Parameter path can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse(path)?;
	}

	pub fn parse_resource(&self, path: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | java.io.IOException | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		Preconditions::check_not_null(path, "Parameter path can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse_resource(path)?;
	}

	pub fn parse_resource(&self, path: &/* Java */ java::lang::String /**/, encoding: &/* Java */ java::nio::charset::Charset /**/) /* thrown(java.lang.AssertionError | java.io.IOException | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		Preconditions::check_not_null(path, "Parameter path can't be null.")?;
		Preconditions::check_not_null(encoding, "Parameter encoding can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::handle_result(&com::github::javaparser::static_java_parser::StaticJavaParser::new_parser().parse_resource(path, encoding)?)?;
	}

	pub fn parse_resource(&self, class_loader: &/* Java */ java::lang::ClassLoader /**/, path: &/* Java */ java::lang::String /**/, encoding: &/* Java */ java::nio::charset::Charset /**/) /* thrown(java.lang.AssertionError | java.io.IOException | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		Preconditions::check_not_null(class_loader, "Parameter classLoader can't be null.")?;
		Preconditions::check_not_null(path, "Parameter path can't be null.")?;
		Preconditions::check_not_null(encoding, "Parameter encoding can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::handle_result(&com::github::javaparser::static_java_parser::StaticJavaParser::new_parser().parse_resource(class_loader, path, encoding)?)?;
	}

	pub fn parse(&self, reader: &/* Java */ java::io::Reader /**/) /* thrown(java.lang.AssertionError | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		Preconditions::check_not_null(reader, "Parameter reader can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse(reader)?;
	}

	pub fn parse(&self, code: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		Preconditions::check_not_null(code, "Parameter code can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse(code)?;
	}

	pub fn parse_block(&self, block_statement: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::stmt::block_stmt::BlockStmt {
		Preconditions::check_not_null(block_statement, "Parameter blockStatement can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse_block(block_statement)?;
	}

	pub fn parse_statement(&self, statement: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::stmt::statement::Statement {
		Preconditions::check_not_null(statement, "Parameter statement can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse_statement(statement)?;
	}

	pub fn parse_import(&self, import_declaration: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::import_declaration::ImportDeclaration {
		Preconditions::check_not_null(import_declaration, "Parameter importDeclaration can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse_import(import_declaration)?;
	}

	pub fn parse_expression<T: com::github::javaparser::ast::expr::expression::Expression>(&self, expression: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> T {
		Preconditions::check_not_null(expression, "Parameter expression can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse_expression(expression)?;
	}

	pub fn parse_annotation(&self, annotation: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::expr::annotation_expr::AnnotationExpr {
		Preconditions::check_not_null(annotation, "Parameter annotation can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse_annotation(annotation)?;
	}

	pub fn parse_annotation_body_declaration(&self, body: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::body::body_declaration::BodyDeclaration {
		Preconditions::check_not_null(body, "Parameter body can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse_annotation_body_declaration(body)?;
	}

	pub fn parse_body_declaration(&self, body: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::body::body_declaration::BodyDeclaration {
		Preconditions::check_not_null(body, "Parameter body can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse_body_declaration(body)?;
	}

	pub fn parse_class_or_interface_type(&self, type: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType {
		Preconditions::check_not_null(type, "Parameter type can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse_class_or_interface_type(type)?;
	}

	pub fn parse_type(&self, type: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::type::type::Type {
		Preconditions::check_not_null(type, "Parameter type can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse_type(type)?;
	}

	pub fn parse_variable_declaration_expr(&self, declaration: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr {
		Preconditions::check_not_null(declaration, "Parameter declaration can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse_variable_declaration_expr(declaration)?;
	}

	pub fn parse_javadoc(&self, content: &/* Java */ java::lang::String /**/, is_markdown_comment: bool) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::javadoc::javadoc::Javadoc {
		Preconditions::check_not_null(content, "Parameter content can't be null.")?;
		return JavadocParser::parse(content, is_markdown_comment);
	}

	pub fn parse_explicit_constructor_invocation_stmt(&self, statement: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt {
		Preconditions::check_not_null(statement, "Parameter statement can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse_explicit_constructor_invocation_stmt(statement)?;
	}

	pub fn parse_name(&self, qualified_name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::expr::name::Name {
		Preconditions::check_not_null(qualified_name, "Parameter qualifiedName can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse_name(qualified_name)?;
	}

	pub fn parse_simple_name(&self, name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::expr::simple_name::SimpleName {
		Preconditions::check_not_null(name, "Parameter name can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse_simple_name(name)?;
	}

	pub fn parse_parameter(&self, parameter: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::body::parameter::Parameter {
		Preconditions::check_not_null(parameter, "Parameter parameter can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse_parameter(parameter)?;
	}

	pub fn parse_package_declaration(&self, package_declaration: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::package_declaration::PackageDeclaration {
		Preconditions::check_not_null(package_declaration, "Parameter packageDeclaration can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse_package_declaration(package_declaration)?;
	}

	pub fn parse_type_declaration(&self, type_declaration: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::body::type_declaration::TypeDeclaration {
		Preconditions::check_not_null(type_declaration, "Parameter typeDeclaration can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse_type_declaration(type_declaration)?;
	}

	pub fn parse_module_declaration(&self, module_declaration: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration {
		Preconditions::check_not_null(module_declaration, "Parameter moduleDeclaration can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse_module_declaration(module_declaration)?;
	}

	pub fn parse_module_directive(&self, module_directive: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::modules::module_directive::ModuleDirective {
		Preconditions::check_not_null(module_directive, "Parameter moduleDirective can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse_module_directive(module_directive)?;
	}

	pub fn parse_type_parameter(&self, type_parameter: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::type::type_parameter::TypeParameter {
		Preconditions::check_not_null(type_parameter, "Parameter typeParameter can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse_type_parameter(type_parameter)?;
	}

	pub fn parse_method_declaration(&self, method_declaration: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
		Preconditions::check_not_null(method_declaration, "Parameter methodDeclaration can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse_method_declaration(method_declaration)?;
	}

	pub fn parse_array_initializer_expr(&self, array_initializer_expr: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError | com.github.javaparser.ParseProblemException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr {
		Preconditions::check_not_null(array_initializer_expr, "Parameter arrayInitializerExpr can't be null.")?;
		return com::github::javaparser::static_java_parser::StaticJavaParser::new_parser_adapted().parse_array_initializer_expr(array_initializer_expr)?;
	}

	fn new_parser(&self) -> com::github::javaparser::java_parser::JavaParser {
		return JavaParser::new(&com::github::javaparser::static_java_parser::StaticJavaParser::get_parser_configuration());
	}

	fn new_parser_adapted(&self) -> com::github::javaparser::java_parser_adapter::JavaParserAdapter {
		return JavaParserAdapter::new(&com::github::javaparser::static_java_parser::StaticJavaParser::new_parser());
	}

	fn handle_result<T: com::github::javaparser::ast::node::Node>(&self, result: &com::github::javaparser::parse_result::ParseResult) /* thrown(com.github.javaparser.ParseProblemException) */ -> T {
		if result.is_successful() {
			return result.get_result().get();
		}
		return Err(ParseProblemException::new(&result.get_problems()));
	}

	fn new() -> com::github::javaparser::static_java_parser::StaticJavaParser {
	}
}