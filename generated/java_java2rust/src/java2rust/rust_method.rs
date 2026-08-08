use javaparser_core::com::github::javaparser::ast::body::MethodDeclaration;
use javaparser_core::com::github::javaparser::ast::type::ReferenceType;
use javaparser_core::com::github::javaparser::resolution::declarations::ResolvedMethodDeclaration;
use javaparser_core::com::github::javaparser::resolution::types::ResolvedType;
use commons_lang3::org::jspecify::annotations::NonNull;
use java::util::HashSet;
use java::util::Set;
use java::util::stream::Collectors;

pub struct RustMethod {
	java: com::github::javaparser::ast::body::method_declaration::MethodDeclaration,
	resolved: com::github::javaparser::resolution::declarations::resolved_method_declaration::ResolvedMethodDeclaration,
	id: /* Java */ java::lang::String /**/,
	visibility: /* Java */ java2rust::RustVisibility /**/,
	name: /* Java */ java::lang::String /**/,
	typarams: java2rust::rust_ty_params::RustTyParams = RustTyParams::new(),
	params: java2rust::rust_params::RustParams,
	thrown: /* Java */ java::util::Set /**/ = HashSet<>::new(),
	calls: java2rust::rust_calls::RustCalls = RustCalls::new(),
	item: java2rust::rust_item::RustItem,
	return_type: /* Java */ java::lang::String /**/,
	body: /* Java */ java::lang::String /**/,
}

impl RustMethod {
	pub fn new(item: &java2rust::rust_item::RustItem, java: &com::github::javaparser::ast::body::method_declaration::MethodDeclaration, resolved: &com::github::javaparser::resolution::declarations::resolved_method_declaration::ResolvedMethodDeclaration) -> java2rust::rust_method::RustMethod {
		self.item = item;
		self.java = java;
		self.resolved =  if resolved == null { java.resolve() } else { resolved };
		self.id = self.resolved.get_qualified_signature();
		self.visibility = RustVisibility::pub(&java.is_public());
		self.name = Java2Rust::camel_case_to_snake_case(&java.get_name_as_string());
		self.params = RustParams::new(RustSelf::java2rust::rust_self::RustSelf::REF, &java.get_parameters());
	}

	pub fn analyze(&mut self, transpiler: &java2rust::java_transpiler::JavaTranspiler, item: &java2rust::rust_item::RustItem) {
		self.typarams.analyze(self.resolved, transpiler);
		self.params.analyze(transpiler);
		self.calls.analyze(transpiler);
		// Assign all thrown errors
		for callee in self.calls.callees {
			self.thrown.addAll(&callee.thrown());
		}
		for ty in self.java.get_thrown_exceptions() {
			self.thrown.add(&ty.resolve());
		}
		// Method analysis
		let success_type: String = transpiler.describe(&self.java.get_type());
		self.return_type =  if self.java.get_type().is_void_type() { " " } else { " -> %s ".formatted(success_type) };
		if self.java.get_body().isPresent() {
			self.body = transpiler.describe(&self.java.get_body().orElse(null), self);
		}
		else {self.body = ";";
		}
	
		if !self.thrown.isEmpty() {
			let error_type: String = self.thrown.stream().map(ResolvedType::describe).sorted().collect(&Collectors::joining(" | "));
			self.return_type = " /* thrown(%s) */%s".formatted(error_type, self.return_type);
		}
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.visibility + "fn " + self.name + self.typarams + self.params + self.return_type + self.body;
	}

	pub fn item(&self) -> java2rust::rust_item::RustItem {
		return self.item;
	}

	pub fn params(&self) -> java2rust::rust_params::RustParams {
		return self.params;
	}

	pub fn calls(&self) -> java2rust::rust_calls::RustCalls {
		return self.calls;
	}

	pub fn thrown(&self) -> /* Java */ java::util::Set /**/ {
		return self.thrown;
	}
}

impl java2rust::i_rust_function::IRustFunction for RustMethod {}