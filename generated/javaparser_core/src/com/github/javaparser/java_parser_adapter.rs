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
use java::io;
use java::nio::file::Path;
use java::util::Objects;

pub struct JavaParserAdapter {
	parser: com::github::javaparser::java_parser::JavaParser,
}

impl JavaParserAdapter {
	pub fn of(&self, parser: &com::github::javaparser::java_parser::JavaParser) -> com::github::javaparser::java_parser_adapter::JavaParserAdapter {
		return JavaParserAdapter::new(parser);
	}

	pub fn new(parser: &com::github::javaparser::java_parser::JavaParser) -> com::github::javaparser::java_parser_adapter::JavaParserAdapter {
		self.parser = Objects::requireNonNull(parser, "A non-null parser should be provided.");
	}

	pub fn get_parser(&self) -> com::github::javaparser::java_parser::JavaParser {
		return self.parser;
	}

	fn handle_result<T: com::github::javaparser::ast::node::Node>(&self, result: &com::github::javaparser::parse_result::ParseResult) /* thrown(com.github.javaparser.ParseProblemException) */ -> T {
		if result.is_successful() {
			return result.get_result().orElse(null);
		}
		return Err(ParseProblemException::new(&result.get_problems()));
	}

	pub fn get_parser_configuration(&self) -> com::github::javaparser::parser_configuration::ParserConfiguration {
		return self.parser.get_parser_configuration();
	}

	pub fn parse(&self, in: &/* Java */ java::io::InputStream /**/) /* thrown(com.github.javaparser.ParseProblemException | java.lang.AssertionError | java.lang.RuntimeException) */ -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		return self.handle_result(&self.get_parser().parse(in)?)?;
	}

	pub fn parse(&self, file: &/* Java */ java::io::File /**/) /* thrown(com.github.javaparser.ParseProblemException | java.io.FileNotFoundException) */ -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		return self.handle_result(&self.get_parser().parse(file)?)?;
	}

	pub fn parse(&self, path: &/* Java */ java::nio::file::Path /**/) /* thrown(com.github.javaparser.ParseProblemException | java.io.IOException) */ -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		return self.handle_result(&self.get_parser().parse(path)?)?;
	}

	pub fn parse(&self, reader: &/* Java */ java::io::Reader /**/) /* thrown(com.github.javaparser.ParseProblemException | java.lang.AssertionError) */ -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		return self.handle_result(&self.get_parser().parse(reader)?)?;
	}

	pub fn parse(&self, code: &/* Java */ java::lang::String /**/) /* thrown(com.github.javaparser.ParseProblemException | java.lang.AssertionError) */ -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		return self.handle_result(&self.get_parser().parse(code)?)?;
	}

	pub fn parse_resource(&self, path: &/* Java */ java::lang::String /**/) /* thrown(com.github.javaparser.ParseProblemException | java.io.IOException | java.lang.AssertionError) */ -> com::github::javaparser::ast::compilation_unit::CompilationUnit {
		return self.handle_result(&self.get_parser().parse_resource(path)?)?;
	}

	pub fn parse_block(&self, block_statement: &/* Java */ java::lang::String /**/) /* thrown(com.github.javaparser.ParseProblemException | java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::block_stmt::BlockStmt {
		return self.handle_result(&self.get_parser().parse_block(block_statement)?)?;
	}

	pub fn parse_statement(&self, statement: &/* Java */ java::lang::String /**/) /* thrown(com.github.javaparser.ParseProblemException | java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::statement::Statement {
		return self.handle_result(&self.get_parser().parse_statement(statement)?)?;
	}

	pub fn parse_import(&self, import_declaration: &/* Java */ java::lang::String /**/) /* thrown(com.github.javaparser.ParseProblemException | java.lang.AssertionError) */ -> com::github::javaparser::ast::import_declaration::ImportDeclaration {
		return self.handle_result(&self.get_parser().parse_import(import_declaration)?)?;
	}

	pub fn parse_expression<T: com::github::javaparser::ast::expr::expression::Expression>(&self, expression: &/* Java */ java::lang::String /**/) /* thrown(com.github.javaparser.ParseProblemException | java.lang.AssertionError) */ -> T {
		return self.handle_result(&self.get_parser().parse_expression(expression)?)?;
	}

	pub fn parse_annotation(&self, annotation: &/* Java */ java::lang::String /**/) /* thrown(com.github.javaparser.ParseProblemException | java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::annotation_expr::AnnotationExpr {
		return self.handle_result(&self.get_parser().parse_annotation(annotation)?)?;
	}

	pub fn parse_annotation_body_declaration(&self, body: &/* Java */ java::lang::String /**/) /* thrown(com.github.javaparser.ParseProblemException | java.lang.AssertionError) */ -> com::github::javaparser::ast::body::body_declaration::BodyDeclaration {
		return self.handle_result(&self.get_parser().parse_annotation_body_declaration(body)?)?;
	}

	pub fn parse_body_declaration(&self, body: &/* Java */ java::lang::String /**/) /* thrown(com.github.javaparser.ParseProblemException | java.lang.AssertionError) */ -> com::github::javaparser::ast::body::body_declaration::BodyDeclaration {
		return self.handle_result(&self.get_parser().parse_body_declaration(body)?)?;
	}

	pub fn parse_class_or_interface_type(&self, type: &/* Java */ java::lang::String /**/) /* thrown(com.github.javaparser.ParseProblemException | java.lang.AssertionError) */ -> com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType {
		return self.handle_result(&self.get_parser().parse_class_or_interface_type(type)?)?;
	}

	pub fn parse_type(&self, type: &/* Java */ java::lang::String /**/) /* thrown(com.github.javaparser.ParseProblemException | java.lang.AssertionError) */ -> com::github::javaparser::ast::type::type::Type {
		return self.handle_result(&self.get_parser().parse_type(type)?)?;
	}

	pub fn parse_variable_declaration_expr(&self, declaration: &/* Java */ java::lang::String /**/) /* thrown(com.github.javaparser.ParseProblemException | java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr {
		return self.handle_result(&self.get_parser().parse_variable_declaration_expr(declaration)?)?;
	}

	pub fn parse_javadoc(&self, content: &/* Java */ java::lang::String /**/, is_markdown_comment: bool) -> com::github::javaparser::javadoc::javadoc::Javadoc {
		return JavadocParser::parse(content, is_markdown_comment);
	}

	pub fn parse_explicit_constructor_invocation_stmt(&self, statement: &/* Java */ java::lang::String /**/) /* thrown(com.github.javaparser.ParseProblemException | java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::explicit_constructor_invocation_stmt::ExplicitConstructorInvocationStmt {
		return self.handle_result(&self.get_parser().parse_explicit_constructor_invocation_stmt(statement)?)?;
	}

	pub fn parse_name(&self, qualified_name: &/* Java */ java::lang::String /**/) /* thrown(com.github.javaparser.ParseProblemException | java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::name::Name {
		return self.handle_result(&self.get_parser().parse_name(qualified_name)?)?;
	}

	pub fn parse_simple_name(&self, name: &/* Java */ java::lang::String /**/) /* thrown(com.github.javaparser.ParseProblemException | java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::simple_name::SimpleName {
		return self.handle_result(&self.get_parser().parse_simple_name(name)?)?;
	}

	pub fn parse_parameter(&self, parameter: &/* Java */ java::lang::String /**/) /* thrown(com.github.javaparser.ParseProblemException | java.lang.AssertionError) */ -> com::github::javaparser::ast::body::parameter::Parameter {
		return self.handle_result(&self.get_parser().parse_parameter(parameter)?)?;
	}

	pub fn parse_package_declaration(&self, package_declaration: &/* Java */ java::lang::String /**/) /* thrown(com.github.javaparser.ParseProblemException | java.lang.AssertionError) */ -> com::github::javaparser::ast::package_declaration::PackageDeclaration {
		return self.handle_result(&self.get_parser().parse_package_declaration(package_declaration)?)?;
	}

	pub fn parse_type_declaration(&self, type_declaration: &/* Java */ java::lang::String /**/) /* thrown(com.github.javaparser.ParseProblemException | java.lang.AssertionError) */ -> com::github::javaparser::ast::body::type_declaration::TypeDeclaration {
		return self.handle_result(&self.get_parser().parse_type_declaration(type_declaration)?)?;
	}

	pub fn parse_module_declaration(&self, module_declaration: &/* Java */ java::lang::String /**/) /* thrown(com.github.javaparser.ParseProblemException | java.lang.AssertionError) */ -> com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration {
		return self.handle_result(&self.get_parser().parse_module_declaration(module_declaration)?)?;
	}

	pub fn parse_module_directive(&self, module_directive: &/* Java */ java::lang::String /**/) /* thrown(com.github.javaparser.ParseProblemException | java.lang.AssertionError) */ -> com::github::javaparser::ast::modules::module_directive::ModuleDirective {
		return self.handle_result(&self.get_parser().parse_module_directive(module_directive)?)?;
	}

	pub fn parse_type_parameter(&self, type_parameter: &/* Java */ java::lang::String /**/) /* thrown(com.github.javaparser.ParseProblemException | java.lang.AssertionError) */ -> com::github::javaparser::ast::type::type_parameter::TypeParameter {
		return self.handle_result(&self.get_parser().parse_type_parameter(type_parameter)?)?;
	}

	pub fn parse_method_declaration(&self, method_declaration: &/* Java */ java::lang::String /**/) /* thrown(com.github.javaparser.ParseProblemException | java.lang.AssertionError) */ -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
		return self.handle_result(&self.get_parser().parse_method_declaration(method_declaration)?)?;
	}

	pub fn parse_array_initializer_expr(&self, array_initializer_expr: &/* Java */ java::lang::String /**/) /* thrown(com.github.javaparser.ParseProblemException | java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr {
		return self.handle_result(&self.get_parser().parse_array_initializer_expr(array_initializer_expr)?)?;
	}
}