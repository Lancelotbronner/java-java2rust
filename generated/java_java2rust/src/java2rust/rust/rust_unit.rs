use javaparser_core::com::github::javaparser::JavaParser;
use javaparser_core::com::github::javaparser::ParseResult;
use javaparser_core::com::github::javaparser::Problem;
use javaparser_core::com::github::javaparser::StaticJavaParser;
use javaparser_core::com::github::javaparser::ast::CompilationUnit;
use crate::java2rust::DeclVisitor;
use crate::java2rust::JavaTranspiler;
use java::io::IOException;
use java::nio::file::Path;
use java::util::StringJoiner;

pub struct RustUnit {
	jar: java2rust::rust::rust_jar::RustJar,
	path: /* Java */ java::nio::file::Path /**/,
	pkg: java2rust::rust::rust_package::RustPackage,
	java: com::github::javaparser::parse_result::ParseResult,
}

impl RustUnit {
	pub fn new(jar: &java2rust::rust::rust_jar::RustJar, pkg: &java2rust::rust::rust_package::RustPackage, path: &/* Java */ java::nio::file::Path /**/) /* thrown(java.io.IOException) */ -> java2rust::rust::rust_unit::RustUnit {
		let parser: JavaParser = JavaParser::new(&StaticJavaParser::get_parser_configuration());
		this(jar, path, pkg, &parser.parse(path)?);
	}

	pub fn new(jar: &java2rust::rust::rust_jar::RustJar, path: &/* Java */ java::nio::file::Path /**/, pkg: &java2rust::rust::rust_package::RustPackage, java: &com::github::javaparser::parse_result::ParseResult) -> java2rust::rust::rust_unit::RustUnit {
		self.jar = jar;
		self.path = path;
		self.pkg = pkg;
		self.java = java;
	}

	pub fn preanalyze(&self, transpiler: &java2rust::java_transpiler::JavaTranspiler) {
		if self.java.get_result().isEmpty() {
			return;
		}
	
		self.java.get_result().get().accept(DeclVisitor::new(transpiler, self.pkg), null);
	}

	pub fn is_successful(&self) -> bool {
		return self.java.is_successful();
	}

	pub fn has_problems(&self) -> bool {
		return !self.java.is_successful();
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		let problems: StringJoiner = StringJoiner::new("\n");
		for problem in self.java.get_problems() {
			problems.add(&"// FIXME: %s".formatted(&problem.get_verbose_message()));
		}
		let problems_string: String = problems.toString();
		let sb: StringBuilder = StringBuilder::new(&self.pkg.to_string());
		if !sb.isEmpty() && !problems_string.isEmpty() {
			sb.append("\n\n");
		}
	
		sb.append(problems_string);
		return sb.toString();
	}
}