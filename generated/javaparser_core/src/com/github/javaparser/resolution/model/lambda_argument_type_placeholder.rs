use crate::com::github::javaparser::resolution::declarations::ResolvedMethodLikeDeclaration;
use crate::com::github::javaparser::resolution::types::ResolvedType;
use java::util::Optional;

pub struct LambdaArgumentTypePlaceholder {
	pos: i32,
	parameter_count: /* Java */ java::util::Optional /**/,
	body_block_has_explicit_non_void_return: /* Java */ java::util::Optional /**/,
	method: com::github::javaparser::resolution::model::symbol_reference::SymbolReference,
}

impl LambdaArgumentTypePlaceholder {
	pub fn new(pos: i32) -> com::github::javaparser::resolution::model::lambda_argument_type_placeholder::LambdaArgumentTypePlaceholder {
		self.pos = pos;
		self.parameterCount = Optional::empty();
		self.bodyBlockHasExplicitNonVoidReturn = Optional::empty();
	}

	pub fn new(pos: i32, parameter_count: i32, body_block_has_explicit_non_void_return: &/* Java */ java::util::Optional /**/) -> com::github::javaparser::resolution::model::lambda_argument_type_placeholder::LambdaArgumentTypePlaceholder {
		self.pos = pos;
		self.parameterCount = Optional::of(parameter_count);
		self.bodyBlockHasExplicitNonVoidReturn = body_block_has_explicit_non_void_return;
	}

	pub fn get_parameter_count(&self) -> /* Java */ java::util::Optional /**/ {
		return self.parameter_count;
	}

	pub fn body_block_has_explicit_non_void_return(&self) -> /* Java */ java::util::Optional /**/ {
		return self.body_block_has_explicit_non_void_return;
	}

	pub fn is_array(&self) -> bool {
		return false;
	}

	pub fn is_reference_type(&self) -> bool {
		return false;
	}

	pub fn describe(&self) /* thrown(java.lang.UnsupportedOperationException) */ -> /* Java */ java::lang::String /**/ {
		return Err(UnsupportedOperationException::new());
	}

	pub fn is_type_variable(&self) -> bool {
		return false;
	}

	pub fn set_method(&mut self, method: &com::github::javaparser::resolution::model::symbol_reference::SymbolReference) {
		self.method = method;
	}

	pub fn is_assignable_by(&self, other: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) /* thrown(java.lang.UnsupportedOperationException) */ -> bool {
		return Err(UnsupportedOperationException::new());
	}
}

impl com::github::javaparser::resolution::types::resolved_type::ResolvedType for LambdaArgumentTypePlaceholder {}