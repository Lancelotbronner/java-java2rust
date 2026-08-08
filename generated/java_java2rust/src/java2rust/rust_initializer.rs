use javaparser_core::com::github::javaparser::ast::body::BodyDeclaration;
use javaparser_core::com::github::javaparser::ast::body::InitializerDeclaration;
use javaparser_core::com::github::javaparser::ast::body::TypeDeclaration;
use javaparser_core::com::github::javaparser::resolution::types::ResolvedType;
use commons_lang3::org::jspecify::annotations::NonNull;
use java::util::HashSet;
use java::util::Set;

pub struct RustInitializer {
	java: com::github::javaparser::ast::body::initializer_declaration::InitializerDeclaration,
	id: /* Java */ java::lang::String /**/,
	thrown: /* Java */ java::util::Set /**/ = HashSet<>::new(),
	calls: java2rust::rust_calls::RustCalls = RustCalls::new(),
	item: java2rust::rust_item::RustItem,
	body: /* Java */ java::lang::String /**/,
}

impl RustInitializer {
	pub fn new(item: &java2rust::rust_item::RustItem, java: &com::github::javaparser::ast::body::initializer_declaration::InitializerDeclaration) -> java2rust::rust_initializer::RustInitializer {
		self.item = item;
		self.java = java;
		let ty: TypeDeclaration<?> = ((java.get_parent_node().get()) as BodyDeclaration<?>).as_type_declaration()?;
		self.id = ty.get_fully_qualified_name().orElse(&ty.get_name_as_string());
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "init %s".formatted(&self.java.get_body());
	}

	pub fn item(&self) -> java2rust::rust_item::RustItem {
		return self.item;
	}

	pub fn params(&self) -> java2rust::rust_params::RustParams {
		return RustParams::java2rust::rust_params::RustParams::EMPTY;
	}

	pub fn calls(&self) -> java2rust::rust_calls::RustCalls {
		return self.calls;
	}

	pub fn thrown(&self) -> /* Java */ java::util::Set /**/ {
		return self.thrown;
	}

	pub fn analyze(&self, transpiler: &java2rust::java_transpiler::JavaTranspiler, item: &java2rust::rust_item::RustItem) {
		self.calls.analyze(transpiler);
		// Assign all thrown errors
		for callee in self.calls.callees {
			self.thrown.addAll(&callee.thrown());
		}
	}
}

impl java2rust::i_rust_function::IRustFunction for RustInitializer {}