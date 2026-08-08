use javaparser_core::com::github::javaparser::JavaParser;
use javaparser_core::com::github::javaparser::StaticJavaParser;
use javaparser_core::com::github::javaparser::utils::SourceZip;
use crate::java2rust::Java2Rust;
use crate::java2rust::JavaTranspiler;
use commons_lang3::org::apache::commons::io::FilenameUtils;
use commons_lang3::org::jspecify::annotations::NonNull;
use commons_lang3::org::jspecify::annotations::Nullable;
use java::io::IOException;
use java::nio::file::Files;
use java::nio::file::Path;
use java::util::ArrayList;
use java::util::List;

pub struct RustJar {
	id: /* Java */ java::lang::String /**/,
	name: /* Java */ java::lang::String /**/,
	path: /* Java */ java::nio::file::Path /**/,
	units: /* Java */ java::util::List /**/ = ArrayList<>::new(),
	lib: java2rust::rust::rust_package::RustPackage,
	main: java2rust::rust::rust_package::RustPackage,
}

impl RustJar {
	pub fn new(id: &/* Java */ java::lang::String /**/, name: &/* Java */ java::lang::String /**/, path: &/* Java */ java::nio::file::Path /**/) -> java2rust::rust::rust_jar::RustJar {
		self.id = id;
		self.name = Java2Rust::camel_case_to_snake_case(name).replace("-", "_");
		self.path = path;
		self.lib = RustPackage::new(self.name, null, RustVisibility::java2rust::rust::rust_visibility::RustVisibility::INFERRED, self);
		self.main = RustPackage::new(self.name, null, RustVisibility::java2rust::rust::rust_visibility::RustVisibility::INFERRED, self);
	}

	pub fn sources(&self, id: &/* Java */ java::lang::String /**/, name: &/* Java */ java::lang::String /**/, jar: &com::github::javaparser::utils::source_zip::SourceZip) /* thrown(java.lang.AssertionError | java.io.IOException) */ -> java2rust::rust::rust_jar::RustJar {
		let crate: RustJar = RustJar::new(id, name, &jar.get_zip_path());
		jar.set_parser_configuration(&StaticJavaParser::get_parser_configuration())?;
		jar.parse(|(relative_zip_entry_path, result)|{
			let pkg: RustPackage = crate.lib;
			for trunk in relative_zip_entry_path {
				pkg = pkg.submodule(&FilenameUtils.removeExtension(&trunk.toFile().getName()), RustVisibility::java2rust::rust::rust_visibility::RustVisibility::PUB);
			}
			let unit: RustUnit = RustUnit::new(crate, &jar.get_zip_path().resolve(relative_zip_entry_path), pkg, result);
			crate.units.add(unit);
		} as SourceZip.Callback)?;
		return crate;
	}

	pub fn add(&self, path: &/* Java */ java::nio::file::Path /**/, pkg: &java2rust::rust::rust_package::RustPackage) /* thrown(java.io.IOException) */ {
		self.units.add(RustUnit::new(self, pkg, path));
	}

	pub fn add_source_code(&self, path: &/* Java */ java::nio::file::Path /**/, pkg: &java2rust::rust::rust_package::RustPackage, code: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ {
		let parser: JavaParser = JavaParser::new(&StaticJavaParser::get_parser_configuration());
		self.units.add(RustUnit::new(self, path, pkg, &parser.parse(code)?));
	}

	pub fn add_script(&self, path: &/* Java */ java::nio::file::Path /**/, pkg: &java2rust::rust::rust_package::RustPackage, code: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ {
		let parser: JavaParser = JavaParser::new(&StaticJavaParser::get_parser_configuration());
		self.units.add(RustUnit::new(self, path, pkg, &parser.parse(code)?));
	}

	pub fn preanalyze(&self, transpiler: &java2rust::java_transpiler::JavaTranspiler) {
		for unit in self.units {
			unit.preanalyze(transpiler);
		}
	}

	pub fn analyze(&self, transpiler: &java2rust::java_transpiler::JavaTranspiler) {
		self.lib.analyze(transpiler);
		self.main.analyze(transpiler);
	}

	pub fn generate(&self, path: &/* Java */ java::nio::file::Path /**/) /* thrown(java.io.IOException) */ {
		let crate: Path = path.resolve(self.name);
		let src: Path = crate.resolve("src");
		Files::createDirectories(src);
		if !self.lib.is_empty() {
			self.lib.generate(src);
		}
	
		if !self.main.is_empty() {
			self.main.generate(src);
		}
	
		//TODO: version, metadata, dependencies, etc.
		Files::writeString(&crate.resolve("Cargo.toml"), &self.cargo());
	}

	pub fn cargo(&self) -> /* Java */ java::lang::String /**/ {
		return r#"
		[package]
		name = "%s"
		version = "0.1.0"
		edition = "2024"
		
		[dependencies]
		"#.formatted(self.name);
	}
}