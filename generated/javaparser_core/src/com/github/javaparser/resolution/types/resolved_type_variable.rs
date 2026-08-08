use crate::com::github::javaparser::resolution::Context;
use crate::com::github::javaparser::resolution::declarations::ResolvedTypeParameterDeclaration;
use java::util::List;
use java::util::Map;

pub struct ResolvedTypeVariable {
	type_parameter: com::github::javaparser::resolution::declarations::resolved_type_parameter_declaration::ResolvedTypeParameterDeclaration,
}

impl ResolvedTypeVariable {
	pub fn new(type_parameter: &com::github::javaparser::resolution::declarations::resolved_type_parameter_declaration::ResolvedTypeParameterDeclaration) -> com::github::javaparser::resolution::types::resolved_type_variable::ResolvedTypeVariable {
		self.typeParameter = type_parameter;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "TypeVariable {" + self.type_parameter.toString() + "}";
	}

	pub fn qualified_name(&self) -> /* Java */ java::lang::String /**/ {
		return self.typeParameter.get_qualified_name();
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if o == null || self.getClass() != o.getClass() {
			return false;
		}
	
		let that: ResolvedTypeVariable = o as ResolvedTypeVariable;
		if !self.type_parameter.get_name().equals(&that.typeParameter.get_name()) {
			return false;
		}
	
		if self.type_parameter.declared_on_type() != that.typeParameter.declared_on_type() {
			return false;
		}
	
		if self.type_parameter.declared_on_method() != that.typeParameter.declared_on_method() {
			return false;
		}
	
		return true;
	}

	pub fn hash_code(&self) -> i32 {
		return self.type_parameter.hashCode();
	}

	pub fn is_array(&self) -> bool {
		return false;
	}

	pub fn replace_type_variables(&self, tp_to_be_replaced: &com::github::javaparser::resolution::declarations::resolved_type_parameter_declaration::ResolvedTypeParameterDeclaration, replaced: &com::github::javaparser::resolution::types::resolved_type::ResolvedType, inferred_types: &/* Java */ java::util::Map /**/) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		if tp_to_be_replaced.get_name().equals(&self.typeParameter.get_name()) {
			inferred_types.put(&self.as_type_parameter(), replaced);
			return replaced;
		}
		return self;
	}

	pub fn is_reference_type(&self) -> bool {
		return false;
	}

	pub fn describe(&self) -> /* Java */ java::lang::String /**/ {
		return self.type_parameter.get_name();
	}

	pub fn as_type_parameter(&self) -> com::github::javaparser::resolution::declarations::resolved_type_parameter_declaration::ResolvedTypeParameterDeclaration {
		return self.type_parameter;
	}

	pub fn as_type_variable(&self) -> com::github::javaparser::resolution::types::resolved_type_variable::ResolvedTypeVariable {
		return self;
	}

	pub fn is_type_variable(&self) -> bool {
		return true;
	}

	pub fn is_assignable_by(&self, other: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) /* thrown(java.lang.UnsupportedOperationException) */ -> bool {
		if other.is_type_variable() {
			// we have to compare the type of the bound. For the moment we are focusing solely on the first type.
			if self.type_parameter.has_bound() && other.as_type_variable()?.as_type_parameter().has_bound() {
				return self.type_parameter.get_bounds().get(0).get_type().is_assignable_by(&other.as_type_variable()?.as_type_parameter().get_bounds().get(0).get_type());
			}
			return self.describe().equals(&other.describe());
		}
		return true;
	}

	pub fn mention(&self, type_parameters: &/* Java */ java::util::List /**/) -> bool {
		return type_parameters.contains(self.type_parameter);
	}

	pub fn erasure(&self) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		if self.type_parameter.is_bounded() {
			return self.type_parameter.get_bounds().get(0).get_type();
		}
		return self.type_parameter.object();
	}

	pub fn solve_generic_types(&self, context: &com::github::javaparser::resolution::context::Context) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		return context.solve_generic_type(&self.describe()).orElse(self);
	}

	pub fn to_descriptor(&self) -> /* Java */ java::lang::String /**/ {
		return String::format("L%s;", &self.qualified_name());
	}
}

impl com::github::javaparser::resolution::types::resolved_type::ResolvedType for ResolvedTypeVariable {}