use javaparser_core::com::github::javaparser::ast::body::Parameter;
use crate::java2rust::Java2Rust;
use crate::java2rust::JavaTranspiler;

pub struct RustParam {
	java: com::github::javaparser::ast::body::parameter::Parameter,
	is_mutable: bool,
	name: /* Java */ java::lang::String /**/,
	type: /* Java */ java::lang::String /**/,
	cache: /* Java */ java::lang::String /**/,
}

impl RustParam {
	pub fn new(java: &com::github::javaparser::ast::body::parameter::Parameter) -> java2rust::rust::rust_param::RustParam {
		self.java = java;
		self.name = Java2Rust::camel_case_to_snake_case(&java.get_name_as_string());
	}

	pub fn analyze(&mut self, transpiler: &java2rust::java_transpiler::JavaTranspiler) {
		self.type = transpiler.describe(&self.java.get_type());
		self.cache = self.to_rust();
	}

	fn to_rust(&self) -> /* Java */ java::lang::String /**/ {
		let sb: StringBuilder = StringBuilder::new();
		if self.is_mutable {
			sb.append("mut ");
		}
	
		sb.append(self.name);
		sb.append(": ");
		if self.java.get_type().is_reference_type() {
			sb.append("&");
		}
	
		sb.append(self.type);
		return sb.toString();
	}

	pub fn to_string(&mut self) -> /* Java */ java::lang::String /**/ {
		if self.cache == null {
			self.cache = self.to_rust();
		}
	
		return self.cache;
	}
}