use javaparser_core::com::github::javaparser::ast::body;
use javaparser_core::com::github::javaparser::ast::expr::Expression;
use javaparser_core::com::github::javaparser::ast::type::Type;
use javaparser_core::com::github::javaparser::resolution::declarations::ResolvedReferenceTypeDeclaration;
use crate::java2rust::JavaTranspiler;
use commons_lang3::org::jspecify::annotations::Nullable;
use java::util::ArrayList;
use java::util::List;

pub struct RustItem {
	name: /* Java */ java::lang::String /**/,
	module: java2rust::rust::rust_package::RustPackage,
	visibility: /* Java */ java2rust::rust::RustVisibility /**/,
	methods: /* Java */ java::util::List /**/ = ArrayList<>::new(),
	statics: /* Java */ java::util::List /**/ = ArrayList<>::new(),
}

impl RustItem {
	fn new(name: &/* Java */ java::lang::String /**/, module: &java2rust::rust::rust_package::RustPackage, visibility: &/* Java */ java2rust::rust::RustVisibility /**/) -> java2rust::rust::rust_item::RustItem {
		self.name = name;
		self.module = module;
		self.visibility = visibility;
	}

	pub fn analyze(&self, transpiler: &java2rust::java_transpiler::JavaTranspiler) {
		for field in self.statics {
			field.analyze(transpiler, self);
		}
		for method in self.methods {
			method.analyze(transpiler, self);
		}
	}

	pub fn id(&self) -> /* Java */ java::lang::String /**/ ;

	pub fn path(&self) -> /* Java */ java::lang::String /**/ ;

	pub fn name(&self) -> /* Java */ java::lang::String /**/ {
		return self.name;
	}

	pub fn field(&self, name: &/* Java */ java::lang::String /**/, type: &com::github::javaparser::ast::type::type::Type, initializer: &com::github::javaparser::ast::expr::expression::Expression) -> java2rust::rust::rust_field::RustField {
		return null;
	}

	pub fn static_field(&self, field: &com::github::javaparser::ast::body::field_declaration::FieldDeclaration, declarator: &com::github::javaparser::ast::body::variable_declarator::VariableDeclarator) -> java2rust::rust::rust_static::RustStatic {
		let tmp: RustStatic = RustStatic::new(self, field, declarator);
		self.statics.add(tmp);
		return tmp;
	}

	pub fn method(&self, java: &com::github::javaparser::ast::body::method_declaration::MethodDeclaration) -> java2rust::rust::rust_method::RustMethod {
		let rust: RustMethod = RustMethod::new(self, java, &java.resolve());
		self.methods.add(rust);
		return rust;
	}

	pub fn constructor(&self, java: &com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration) -> java2rust::rust::rust_constructor::RustConstructor {
		let rust: RustConstructor = RustConstructor::new(self, java, &java.resolve());
		self.methods.add(rust);
		return rust;
	}

	pub fn initializer(&self, java: &com::github::javaparser::ast::body::initializer_declaration::InitializerDeclaration) -> java2rust::rust::rust_initializer::RustInitializer {
		let rust: RustInitializer = RustInitializer::new(self, java);
		self.methods.add(rust);
		return rust;
	}
}