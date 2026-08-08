use crate::com::github::javaparser::resolution::declarations::ResolvedTypeParameterDeclaration;
use crate::com::github::javaparser::resolution::types;
use java::util::Map;
use java::util::function::Function;

pub struct LazyType {
	concrete: com::github::javaparser::resolution::types::resolved_type::ResolvedType,
	provider: /* Java */ java::util::function::Function /**/,
}

impl LazyType {
	pub fn new(provider: &/* Java */ java::util::function::Function /**/) -> com::github::javaparser::resolution::model::typesystem::lazy_type::LazyType {
		self.provider = provider;
	}

	pub fn get_type(&mut self) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		if self.concrete == null {
			self.concrete = self.provider.apply(null);
		}
		return self.concrete;
	}

	pub fn is_array(&self) -> bool {
		return self.get_type().is_array();
	}

	pub fn array_level(&self) -> i32 {
		return self.get_type().array_level();
	}

	pub fn is_primitive(&self) -> bool {
		return self.get_type().is_primitive();
	}

	pub fn is_null(&self) -> bool {
		return self.get_type().is_null();
	}

	pub fn is_reference(&self) -> bool {
		return self.get_type().is_reference();
	}

	pub fn is_reference_type(&self) -> bool {
		return self.get_type().is_reference_type();
	}

	pub fn is_void(&self) -> bool {
		return self.get_type().is_void();
	}

	pub fn is_type_variable(&self) -> bool {
		return self.get_type().is_type_variable();
	}

	pub fn is_wildcard(&self) -> bool {
		return self.get_type().is_wildcard();
	}

	pub fn as_array_type(&self) /* thrown(java.lang.UnsupportedOperationException) */ -> com::github::javaparser::resolution::types::resolved_array_type::ResolvedArrayType {
		return self.get_type().as_array_type()?;
	}

	pub fn as_reference_type(&self) /* thrown(java.lang.UnsupportedOperationException) */ -> com::github::javaparser::resolution::types::resolved_reference_type::ResolvedReferenceType {
		return self.get_type().as_reference_type()?;
	}

	pub fn as_type_parameter(&self) /* thrown(java.lang.UnsupportedOperationException) */ -> com::github::javaparser::resolution::declarations::resolved_type_parameter_declaration::ResolvedTypeParameterDeclaration {
		return self.get_type().as_type_parameter()?;
	}

	pub fn as_type_variable(&self) /* thrown(java.lang.UnsupportedOperationException) */ -> com::github::javaparser::resolution::types::resolved_type_variable::ResolvedTypeVariable {
		return self.get_type().as_type_variable()?;
	}

	pub fn as_primitive(&self) /* thrown(java.lang.UnsupportedOperationException) */ -> com::github::javaparser::resolution::types::resolved_primitive_type::ResolvedPrimitiveType {
		return self.get_type().as_primitive()?;
	}

	pub fn as_wildcard(&self) /* thrown(java.lang.UnsupportedOperationException) */ -> com::github::javaparser::resolution::types::resolved_wildcard::ResolvedWildcard {
		return self.get_type().as_wildcard()?;
	}

	pub fn describe(&self) -> /* Java */ java::lang::String /**/ {
		return self.get_type().describe();
	}

	pub fn replace_type_variables(&self, tp: &com::github::javaparser::resolution::declarations::resolved_type_parameter_declaration::ResolvedTypeParameterDeclaration, replaced: &com::github::javaparser::resolution::types::resolved_type::ResolvedType, inferred_types: &/* Java */ java::util::Map /**/) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		return self.get_type().replace_type_variables(tp, replaced, inferred_types);
	}

	pub fn replace_type_variables(&self, tp: &com::github::javaparser::resolution::declarations::resolved_type_parameter_declaration::ResolvedTypeParameterDeclaration, replaced: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		return self.get_type().replace_type_variables(tp, replaced);
	}

	pub fn is_assignable_by(&self, other: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) -> bool {
		return self.get_type().is_assignable_by(other);
	}
}

impl com::github::javaparser::resolution::types::resolved_type::ResolvedType for LazyType {}