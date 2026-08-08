use crate::com::github::javaparser::resolution::Context;
use crate::com::github::javaparser::resolution::declarations::ResolvedTypeParameterDeclaration;
use java::util::List;
use java::util::Map;

pub struct ResolvedWildcard {
	type: com::github::javaparser::resolution::types::resolved_wildcard::BoundType,
	bounded_type: com::github::javaparser::resolution::types::resolved_type::ResolvedType,
}

impl ResolvedWildcard {
	pub static UNBOUNDED: com::github::javaparser::resolution::types::resolved_wildcard::ResolvedWildcard = ResolvedWildcard::new(null, null);

	fn new(type: &com::github::javaparser::resolution::types::resolved_wildcard::BoundType, bounded_type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::resolution::types::resolved_wildcard::ResolvedWildcard {
		if type == null && bounded_type != null {
			return Err(IllegalArgumentException::new());
		}
		if type != null && bounded_type == null {
			return Err(IllegalArgumentException::new());
		}
		self.type = type;
		self.boundedType = bounded_type;
	}

	pub fn super_bound(&self, type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) -> com::github::javaparser::resolution::types::resolved_wildcard::ResolvedWildcard {
		return ResolvedWildcard::new(BoundType::SUPER, type);
	}

	pub fn extends_bound(&self, type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) -> com::github::javaparser::resolution::types::resolved_wildcard::ResolvedWildcard {
		return ResolvedWildcard::new(BoundType::EXTENDS, type);
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "WildcardUsage{" + "type=" + self.type + ", boundedType=" + self.bounded_type + '}';
	}

	pub fn is_wildcard(&self) -> bool {
		return true;
	}

	pub fn as_wildcard(&self) -> com::github::javaparser::resolution::types::resolved_wildcard::ResolvedWildcard {
		return self;
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if !(o instanceof ResolvedWildcard) {
			return false;
		}
	
		let that: ResolvedWildcard = o as ResolvedWildcard;
		if  if self.bounded_type != null { !self.bounded_type.equals(that.boundedType) } else { that.boundedType != null } {
			return false;
		}
	
		if self.type != that.type {
			return false;
		}
	
		return true;
	}

	pub fn hash_code(&self) -> i32 {
		let result: i32 =  if self.type != null { self.type.hashCode() } else { 0 };
		result = 31 * result + ( if self.bounded_type != null { self.bounded_type.hashCode() } else { 0 });
		return result;
	}

	pub fn describe(&self) /* thrown(java.lang.UnsupportedOperationException) */ -> /* Java */ java::lang::String /**/ {
		if self.type == null {
			return "?";
		}
		if self.type == BoundType::SUPER {
			return "? super " + self.bounded_type.describe();
		}
		if self.type == BoundType::EXTENDS {
			return "? extends " + self.bounded_type.describe();
		}
		return Err(UnsupportedOperationException::new());
	}

	pub fn is_super(&self) -> bool {
		return self.type == BoundType::SUPER;
	}

	pub fn is_extends(&self) -> bool {
		return self.type == BoundType::EXTENDS;
	}

	pub fn is_bounded(&self) -> bool {
		return self.is_super() || self.is_extends();
	}

	pub fn get_bounded_type(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		if self.bounded_type == null {
			return Err(IllegalStateException::new());
		}
		return self.bounded_type;
	}

	pub fn is_assignable_by(&self, other: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) /* thrown(java.lang.RuntimeException) */ -> bool {
		if self.bounded_type == null {
			// other.asReferenceType().getQualifiedName().equals(Object.class.getCanonicalName());
			return false;
		}
		if self.type == BoundType::SUPER {
			return self.bounded_type.is_assignable_by(other);
		}
		if self.type == BoundType::EXTENDS {
			return false;
		}
		return Err(RuntimeException::new());
	}

	pub fn replace_type_variables(&self, tp_to_replace: &com::github::javaparser::resolution::declarations::resolved_type_parameter_declaration::ResolvedTypeParameterDeclaration, replaced: &com::github::javaparser::resolution::types::resolved_type::ResolvedType, inferred_types: &/* Java */ java::util::Map /**/) /* thrown(java.lang.RuntimeException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		if replaced == null {
			return Err(IllegalArgumentException::new());
		}
		if self.bounded_type == null {
			return self;
		}
		let bounded_type_replaced: ResolvedType = self.bounded_type.replace_type_variables(tp_to_replace, replaced, inferred_types);
		if bounded_type_replaced == null {
			return Err(RuntimeException::new());
		}
		if bounded_type_replaced != self.bounded_type {
			return ResolvedWildcard::new(self.type, bounded_type_replaced);
		}
		return self;
	}

	pub fn mention(&self, type_parameters: &/* Java */ java::util::List /**/) /* thrown(java.lang.UnsupportedOperationException) */ -> bool {
		return self.bounded_type != null && self.bounded_type.mention(type_parameters)?;
	}

	pub fn is_upper_bounded(&self) -> bool {
		return self.is_extends();
	}

	pub fn is_lower_bounded(&self) -> bool {
		return self.is_super();
	}

	pub fn solve_generic_types(&self, context: &com::github::javaparser::resolution::context::Context) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		if self.is_extends() || self.is_super() {
			let bound_resolved: ResolvedType = self.get_bounded_type()?.solve_generic_types(context);
			if self.is_extends() {
				return ResolvedWildcard::extends_bound(bound_resolved);
			}
			return ResolvedWildcard::super_bound(bound_resolved);
		}
		return self;
	}

	pub fn erasure(&self) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		return self.bounded_type;
	}
}

impl com::github::javaparser::resolution::types::resolved_type::ResolvedType for ResolvedWildcard {}

pub enum BoundType;