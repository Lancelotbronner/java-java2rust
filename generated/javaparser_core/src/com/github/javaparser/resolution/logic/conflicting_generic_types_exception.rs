use crate::com::github::javaparser::resolution::types::ResolvedType;

pub struct ConflictingGenericTypesException;

impl ConflictingGenericTypesException {
	pub fn new(formal_type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType, actual_type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) -> com::github::javaparser::resolution::logic::conflicting_generic_types_exception::ConflictingGenericTypesException {
		super(&String::format("No matching between %s (formal) and %s (actual)", formal_type, actual_type));
	}
}

impl /* Java */ java::io::Serializable /**/ for ConflictingGenericTypesException {}