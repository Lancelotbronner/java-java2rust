use javaparser_core::com::github::javaparser::ast::ImportDeclaration;
use javaparser_core::com::github::javaparser::ast::expr::Name;
use crate::java2rust::JavaTranspiler;
use commons_lang3::org::apache::commons::lang3::StringUtils;
use commons_lang3::org::jspecify::annotations::Nullable;
use java::util::Arrays;
use java::util::Optional;
use java::util::stream::Stream;

pub struct RustImport {
	java: com::github::javaparser::ast::import_declaration::ImportDeclaration,
	module: java2rust::rust::rust_package::RustPackage,
	path: /* Java */ java::lang::String /**/,
	crate: java2rust::rust::rust_jar::RustJar,
}

impl RustImport {
	pub fn new(java: &com::github::javaparser::ast::import_declaration::ImportDeclaration, mod: &java2rust::rust::rust_package::RustPackage) -> java2rust::rust::rust_import::RustImport {
		self.java = java;
		self.module = mod;
		self.path = java.get_name_as_string().replace(".", "::");
	}

	pub fn analyze(&mut self, transpiler: &java2rust::java_transpiler::JavaTranspiler) {
		let pkg: RustPackage = transpiler.locate(&self.java.get_name());
		if pkg != null {
			self.crate = pkg.crate;
		}
	
	}

	pub fn to_string(&self, crate: &java2rust::rust::rust_jar::RustJar) -> /* Java */ java::lang::String /**/ {
		let root: String;
		if self.crate == null {
			//Note: We're assuming this was obtained via reflection
			return "use %s;".formatted(self.path);
		}
	
		if crate == self.crate {
			root = "crate";
		}
		else {root = self.crate.name;
		}
	
		return "use %s::%s;".formatted(root, self.path);
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "use %s;".formatted(self.path);
	}
}