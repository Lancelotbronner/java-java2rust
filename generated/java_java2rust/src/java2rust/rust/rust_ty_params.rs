use javaparser_core::com::github::javaparser::resolution::declarations::ResolvedTypeParameterDeclaration;
use javaparser_core::com::github::javaparser::resolution::declarations::ResolvedTypeParametrizable;
use javaparser_core::com::github::javaparser::utils::Pair;
use crate::java2rust::JavaTranspiler;
use java::util::ArrayList;
use java::util::List;
use java::util::StringJoiner;

pub struct RustTyParams {
	params: /* Java */ java::util::ArrayList /**/ = ArrayList<>::new(),
	impl_cache: /* Java */ java::lang::String /**/,
	bounds_cache: /* Java */ java::lang::String /**/,
}

impl RustTyParams {
	pub fn new() -> java2rust::rust::rust_ty_params::RustTyParams {
	}

	pub fn analyze(&self, decl: &com::github::javaparser::resolution::declarations::resolved_type_parametrizable::ResolvedTypeParametrizable, transpiler: &java2rust::java_transpiler::JavaTranspiler) {
		self.analyze(&decl.get_type_parameters(), transpiler);
	}

	fn analyze(&mut self, typarams: &/* Java */ java::util::List /**/, transpiler: &java2rust::java_transpiler::JavaTranspiler) /* thrown(java.lang.UnsupportedOperationException) */ {
		for param in typarams {
			let r0 = 'try0: {
				let bounds: StringJoiner = StringJoiner::new("+");
				for bound in param.get_bounds() {
					bounds.add(&match transpiler.describe(&bound.get_type()) {
						Err(e) => break 'try0 Err(e),
						Ok(s) => s,
					});
				}
				self.params.add(Pair<>::new(&param.get_name(), &bounds.toString()));
				break 'try0 Ok(());
			};
			match r0 {
				Err(e @ Throwable) => {
					System::err.println(e);
					self.params.add(Pair<>::new(&param.get_name(), &"/* %s */".formatted(&e.getMessage())));
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
		}
		self.impl_cache = self.to_rust_impl();
		self.bounds_cache = self.to_rust_bounds();
	}

	fn to_rust_impl(&self) -> /* Java */ java::lang::String /**/ {
		let params: StringJoiner = StringJoiner::new(", ", "<", ">");
		params.setEmptyValue("");
		for param in self.params {
			params.add(param.a);
		}
		return params.toString();
	}

	fn to_rust_bounds(&self) -> /* Java */ java::lang::String /**/ {
		let params: StringJoiner = StringJoiner::new(", ", "<", ">");
		params.setEmptyValue("");
		for param in self.params {
			let sb: StringBuilder = StringBuilder::new();
			sb.append(param.a);
			if !param.b.isEmpty() {
				sb.append(": ");
				sb.append(param.b);
			}
			params.add(&sb.toString());
		}
		return params.toString();
	}

	pub fn to_impl(&mut self) -> /* Java */ java::lang::String /**/ {
		if self.impl_cache == null {
			self.impl_cache = self.to_rust_impl();
		}
	
		return self.impl_cache;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.to_bounds();
	}

	pub fn to_bounds(&mut self) -> /* Java */ java::lang::String /**/ {
		if self.bounds_cache == null {
			self.bounds_cache = self.to_rust_bounds();
		}
	
		return self.bounds_cache;
	}
}