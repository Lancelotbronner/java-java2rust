use crate::com::github::javaparser::resolution::declarations::ResolvedTypeParameterDeclaration;
use java::util::Map;

pub struct ResolvedArrayType {
	base_type: com::github::javaparser::resolution::types::resolved_type::ResolvedType,
}

impl ResolvedArrayType {
	pub fn new(base_type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) -> com::github::javaparser::resolution::types::resolved_array_type::ResolvedArrayType {
		self.baseType = base_type;
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if o == null || self.getClass() != o.getClass() {
			return false;
		}
	
		let that: ResolvedArrayType = o as ResolvedArrayType;
		if !self.base_type.equals(that.baseType) {
			return false;
		}
	
		return true;
	}

	pub fn hash_code(&self) -> i32 {
		return self.base_type.hashCode();
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "ResolvedArrayType{" + self.base_type + "}";
	}

	pub fn as_array_type(&self) -> com::github::javaparser::resolution::types::resolved_array_type::ResolvedArrayType {
		return self;
	}

	pub fn is_array(&self) -> bool {
		return true;
	}

	pub fn describe(&self) -> /* Java */ java::lang::String /**/ {
		return self.base_type.describe() + "[]";
	}

	pub fn get_component_type(&self) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		return self.base_type;
	}

	pub fn is_assignable_by(&self, other: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) /* thrown(java.lang.UnsupportedOperationException) */ -> bool {
		if other.is_null() {
			return true;
		}
		if other.is_array() {
			if self.base_type.is_primitive() && other.as_array_type()?.get_component_type().is_primitive() {
				return self.base_type.equals(&other.as_array_type()?.get_component_type());
			}
			// An array of primitive type cannot be assigned to an array of Object
			if (self.base_type.is_primitive() && other.as_array_type()?.get_component_type().is_reference_type()) || (self.base_type.is_reference_type() && other.as_array_type()?.get_component_type().is_primitive()) {
				return false;
			}
			// a variable of type Object, Cloneable or java.io.Serializable.
			return self.base_type.is_assignable_by(&other.as_array_type()?.get_component_type());
		}
		return false;
	}

	pub fn replace_type_variables(&self, tp_to_replace: &com::github::javaparser::resolution::declarations::resolved_type_parameter_declaration::ResolvedTypeParameterDeclaration, replaced: &com::github::javaparser::resolution::types::resolved_type::ResolvedType, inferred_types: &/* Java */ java::util::Map /**/) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		let base_type_replaced: ResolvedType = self.base_type.replace_type_variables(tp_to_replace, replaced, inferred_types);
		if base_type_replaced == self.base_type {
			return self;
		}
		return ResolvedArrayType::new(base_type_replaced);
	}

	pub fn erasure(&self) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		return ResolvedArrayType::new(&self.base_type.erasure());
	}

	pub fn to_descriptor(&self) -> /* Java */ java::lang::String /**/ {
		let sb: StringBuffer = StringBuffer::new();
		sb.append("[");
		sb.append(&self.base_type.to_descriptor());
		return sb.toString();
	}
}

impl com::github::javaparser::resolution::types::resolved_type::ResolvedType for ResolvedArrayType {}