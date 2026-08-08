pub struct ResolvedVoidType;

impl ResolvedVoidType {
	pub static INSTANCE: com::github::javaparser::resolution::types::resolved_type::ResolvedType = ResolvedVoidType::new();

	fn new() -> com::github::javaparser::resolution::types::resolved_void_type::ResolvedVoidType {
	}

	pub fn describe(&self) -> /* Java */ java::lang::String /**/ {
		return "void";
	}

	pub fn is_assignable_by(&self, other: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) -> bool {
		// In short, nothing can be assign to "void".
		return false;
	}

	pub fn is_void(&self) -> bool {
		return true;
	}

	pub fn to_descriptor(&self) -> /* Java */ java::lang::String /**/ {
		return "V";
	}
}

impl com::github::javaparser::resolution::types::resolved_type::ResolvedType for ResolvedVoidType {}