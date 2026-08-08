use javaparser_core::com::github::javaparser::ast::expr::Expression;
use javaparser_core::com::github::javaparser::ast::type::Type;
use commons_lang3::org::jspecify::annotations::Nullable;

pub struct RustField {
	name: /* Java */ java::lang::String /**/,
	java_type: com::github::javaparser::ast::type::type::Type,
	java_initializer: com::github::javaparser::ast::expr::expression::Expression,
	rust_type: /* Java */ java::lang::String /**/,
	rust_initializer: /* Java */ java::lang::String /**/,
}

impl RustField {
	fn new(name: &/* Java */ java::lang::String /**/, java_type: &com::github::javaparser::ast::type::type::Type, java_initializer: &com::github::javaparser::ast::expr::expression::Expression) -> java2rust::rust_field::RustField {
		self.name = Java2Rust::camel_case_to_snake_case(name);
		self.javaType = java_type;
		self.javaInitializer = java_initializer;
	}

	pub fn analyze(&mut self, transpiler: &java2rust::java_transpiler::JavaTranspiler, item: &java2rust::rust_item::RustItem) /* thrown(java.lang.UnsupportedOperationException) */ {
		//TODO: use item's module to shorten the type via the imports
		//TODO: that means the transpiler needs something that can convert a type to an ID path?
		self.rust_type = transpiler.describe(self.java_type)?;
		if self.java_initializer != null {
			self.rust_initializer = transpiler.describe(self.java_initializer);
		}
	
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		let type: String =  if self.rust_type == null { self.java_type.to_string() } else { self.rust_type };
		let initializer: String =  if self.rust_initializer == null { ( if self.java_initializer == null { null } else { self.java_initializer.to_string() }) } else { self.rust_initializer };
		if initializer == null {
			return "%s: %s".formatted(self.name, type);
		}
	
		return "%s: %s = %s".formatted(self.name, type, initializer);
	}
}