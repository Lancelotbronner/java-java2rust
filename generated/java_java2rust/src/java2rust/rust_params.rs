use javaparser_core::com::github::javaparser::ast::body::Parameter;
use commons_lang3::org::jspecify::annotations::Nullable;
use java::util::Collections;
use java::util::List;
use java::util::Objects;
use java::util::StringJoiner;

pub struct RustParams {
	params: /* Java */ java::util::List /**/,
	self: /* Java */ java2rust::RustSelf /**/,
	cache: /* Java */ java::lang::String /**/,
}

impl RustParams {
	pub static EMPTY: java2rust::rust_params::RustParams = RustParams::new(null, &Collections::emptyList());

	pub fn new(self: &/* Java */ java2rust::RustSelf /**/, java: &/* Java */ java::util::List /**/) -> java2rust::rust_params::RustParams {
		self.self = self;
		self.params = java.stream().map(RustParam::new).toList();
	}

	pub fn analyze(&mut self, transpiler: &java2rust::java_transpiler::JavaTranspiler) /* thrown(java.lang.UnsupportedOperationException) */ {
		for param in self.params {
			param.analyze(transpiler)?;
		}
		self.cache = self.to_rust();
	}

	fn to_rust(&self) -> /* Java */ java::lang::String /**/ {
		let params: StringJoiner = StringJoiner::new(", ", "(", ")");
		if self.self != null {
			params.add(&self.self.template());
		}
	
		for param in self.params {
			params.add(&param.to_string());
		}
		return params.toString();
	}

	pub fn is_mut_self(&self) -> bool {
		if self.self == null {
			return false;
		}
	
		return self.self.isMut();
	}

	pub fn mutate_self(&mut self) {
		if self.self == null {
			return;
		}
	
		self.self = self.self.mut().get();
	}

	pub fn java(&self, name: &/* Java */ java::lang::String /**/) -> java2rust::rust_param::RustParam {
		return self.params.stream().filter(|p|Objects::equals(&p.java.get_name_as_string(), name)).findFirst().orElse(null);
	}

	pub fn java(&self, parameter: &com::github::javaparser::ast::body::parameter::Parameter) -> java2rust::rust_param::RustParam {
		return self.params.stream().filter(|p|p.java == parameter).findFirst().orElse(null);
	}

	pub fn to_string(&mut self) -> /* Java */ java::lang::String /**/ {
		if self.cache == null {
			self.cache = self.to_rust();
		}
	
		return self.cache;
	}
}