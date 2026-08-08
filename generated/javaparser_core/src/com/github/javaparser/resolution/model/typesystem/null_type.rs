use crate::com::github::javaparser::resolution::types::ResolvedType;

pub struct NullType;

impl NullType {
	pub static INSTANCE: com::github::javaparser::resolution::model::typesystem::null_type::NullType = NullType::new();

	fn new() -> com::github::javaparser::resolution::model::typesystem::null_type::NullType {
	// prevent instantiation
	}

	pub fn is_array(&self) -> bool {
		return false;
	}

	pub fn is_null(&self) -> bool {
		return true;
	}

	pub fn is_reference_type(&self) -> bool {
		return false;
	}

	pub fn describe(&self) -> /* Java */ java::lang::String /**/ {
		return "null";
	}

	pub fn is_type_variable(&self) -> bool {
		return false;
	}

	pub fn is_assignable_by(&self, other: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) /* thrown(java.lang.UnsupportedOperationException) */ -> bool {
		return Err(UnsupportedOperationException::new("It does not make sense to assign a value to null, it can only be assigned"));
	}
}

impl com::github::javaparser::resolution::types::resolved_type::ResolvedType for NullType {}