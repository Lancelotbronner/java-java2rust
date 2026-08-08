use crate::com::github::javaparser::resolution::types::ResolvedReferenceType;
use crate::com::github::javaparser::resolution::types::ResolvedType;
use java::util::List;
use java::util::Optional;

pub trait ResolvedTypeParameterDeclaration;

struct Bound {
	extends_bound: bool,
	type: com::github::javaparser::resolution::types::resolved_type::ResolvedType,
}

impl Bound {
	fn new(extends_bound: bool, type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) -> com::github::javaparser::resolution::declarations::resolved_type_parameter_declaration::Bound {
		self.extendsBound = extends_bound;
		self.type = type;
	}

	pub fn extends_bound(&self, type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) -> com::github::javaparser::resolution::declarations::resolved_type_parameter_declaration::Bound {
		return Bound::new(true, type);
	}

	pub fn super_bound(&self, type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) -> com::github::javaparser::resolution::declarations::resolved_type_parameter_declaration::Bound {
		return Bound::new(false, type);
	}

	pub fn get_type(&self) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		return self.type;
	}

	pub fn is_extends(&self) -> bool {
		return self.extends_bound;
	}

	pub fn is_super(&self) -> bool {
		return !self.is_extends();
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "Bound{" + "extendsBound=" + self.extends_bound + ", type=" + self.type + '}';
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if o == null || self.getClass() != o.getClass() {
			return false;
		}
	
		let bound: Bound = o as Bound;
		if self.extends_bound != bound.extendsBound {
			return false;
		}
	
		return  if self.type != null { self.type.equals(bound.type) } else { bound.type == null };
	}

	pub fn hash_code(&self) -> i32 {
		let result: i32 = ( if self.extends_bound { 1 } else { 0 });
		result = 31 * result + ( if self.type != null { self.type.hashCode() } else { 0 });
		return result;
	}
}